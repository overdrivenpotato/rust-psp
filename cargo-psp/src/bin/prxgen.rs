use clap::Parser;
use goblin::elf32::{
    header::Header,
    program_header::{ProgramHeader, PT_LOAD},
    reloc::{Rel, R_MIPS_GPREL16, R_MIPS_PC16, SIZEOF_REL},
    section_header::{SectionHeader, SHF_ALLOC, SHT_LOPROC, SHT_REL, SHT_SYMTAB},
    sym::{Sym, SIZEOF_SYM},
};
use scroll::{
    ctx::{TryFromCtx, TryIntoCtx},
    Endian,
};
use std::{collections::HashMap, ffi::CStr, fs, path::PathBuf};

const ELF_EXEC_TYPE: u16 = 0x0002;
const ELF_MACHINE_MIPS: u16 = 0x0008;

const PRX_EXEC_TYPE: u16 = 0xFFA0;
const PRX_SHT_REL: u32 = SHT_LOPROC | 0xA0;

#[derive(Parser, Debug)]
#[command(
    name = "prxgen",
    author = "Marko Mijalkovic <marko.mijalkovic97@gmail.com>, Josh Wood <sk83rjosh@gmail.com>",
    version = "0.1",
    about = "Convert PSP ELF files to PRX format"
)]
struct Args {
    #[arg(name = "in_file.elf", help = "Input ELF file")]
    in_file: PathBuf,
    #[arg(name = "out_file.prx", help = "Output PRX file")]
    out_file: PathBuf,
    #[arg(
        default_value = ".rodata.sceModuleInfo",
        help = "Alternative name for .rodata.sceModuleInfo section"
    )]
    minfo: String,
}

fn main() {
    let args = Args::parse();
    PrxBuilder::new(args.in_file, &args.minfo)
        .modify()
        .save(args.out_file);
}

struct PrxBuilder<'a> {
    mod_info_sh_name: &'a str,
    elf_bytes: Vec<u8>,
    header: Header,
    section_headers: Vec<SectionHeader>,
    program_headers: Vec<ProgramHeader>,
    relocations: HashMap<usize, Vec<Rel>>,
}

impl<'a> PrxBuilder<'a> {
    /// Load the input ELF file and parse important structures.
    fn new(path: PathBuf, mod_info_sh_name: &'a str) -> Self {
        let elf_bytes = fs::read(path).unwrap();
        let header = Header::parse(&elf_bytes).unwrap();

        // Validate ELF header.
        assert_eq!(header.e_type, ELF_EXEC_TYPE);
        assert_eq!(header.e_machine, ELF_MACHINE_MIPS);
        assert!(header.e_shstrndx < header.e_shnum);

        let section_headers = SectionHeader::from_bytes(
            &elf_bytes[header.e_shoff as usize..],
            header.e_shnum as usize,
        );
        let program_headers = ProgramHeader::from_bytes(
            &elf_bytes[header.e_phoff as usize..],
            header.e_phnum as usize,
        );

        let relocations: HashMap<usize, Vec<Rel>> = section_headers
            .iter()
            .enumerate()
            .filter(|(_, sh)| {
                if sh.sh_type == SHT_REL || sh.sh_type == PRX_SHT_REL {
                    let sh_target = section_headers[sh.sh_info as usize];
                    sh_target.sh_flags & SHF_ALLOC != 0
                } else {
                    false
                }
            })
            .map(|(i, sh)| {
                let start_idx = sh.sh_offset as usize;
                let end_idx = sh.sh_size as usize + start_idx;
                let relocs = elf_bytes[start_idx..end_idx]
                    .chunks(SIZEOF_REL)
                    .map(|rel_bytes| Rel::try_from_ctx(rel_bytes, Endian::Little).unwrap().0)
                    .collect();

                (i, relocs)
            })
            .collect();
        assert!(!relocations.is_empty());

        Self {
            mod_info_sh_name,
            elf_bytes,
            header,
            section_headers,
            program_headers,
            relocations,
        }
    }

    /// Modify the inner structures to create a PRX format file.
    fn modify(mut self) -> Self {
        // Change ELF type, and program header count.
        self.header.e_type = PRX_EXEC_TYPE;
        self.header.e_phnum = 1;

        // Change all relocation types.
        for (i, rels) in &mut self.relocations {
            let relocation_header = &self.section_headers[*i];

            // Don't touch relocations with invalid links.
            let Some(symbols_header) =
                &self.section_headers.get(relocation_header.sh_link as usize)
            else {
                continue;
            };

            // Don't touch relocations without symbols.
            if symbols_header.sh_type != SHT_SYMTAB {
                continue;
            }

            // Load symbols.
            let symbols = {
                let start_idx = symbols_header.sh_offset as usize;
                let end_idx = symbols_header.sh_size as usize + start_idx;
                self.elf_bytes[start_idx..end_idx]
                    .chunks(SIZEOF_SYM)
                    .map(|rel_bytes| Sym::try_from_ctx(rel_bytes, Endian::Little).unwrap().0)
                    .collect::<Vec<Sym>>()
            };

            // Remove weak relocations.
            rels.retain(|rel| {
                // 16-bit relocs are unsupported.
                if matches!(rel.r_info & 0xFF, R_MIPS_GPREL16 | R_MIPS_PC16) {
                    false
                // relocs outside of section zero must be removed.
                } else if let Some(symbol) = symbols.get((rel.r_info >> 8) as usize) {
                    symbol.st_shndx != 0
                // relocs with invalid symbols must be removed.
                } else {
                    false
                }
            });

            // Set upper 24 bits to 0 (OFS_BASE, ADDR_BASE).
            for rel in rels {
                rel.r_info &= 0xff;
            }
        }

        // Update all relocation headers.
        for (i, rels) in &mut self.relocations {
            let section_header = &mut self.section_headers[*i];
            section_header.sh_type = PRX_SHT_REL;
            section_header.sh_size = (rels.len() * SIZEOF_REL) as u32;
        }

        // Get module info
        let module_info = {
            let sh_string_table = self.section_headers[self.header.e_shstrndx as usize];
            let start_idx = sh_string_table.sh_offset as usize;
            let end_idx = start_idx + sh_string_table.sh_size as usize;
            let section_names = &self.elf_bytes[start_idx..end_idx];
            let section_name = self.mod_info_sh_name;

            self.section_headers.iter().find(|sh| {
                CStr::from_bytes_until_nul(&section_names[sh.sh_name as usize..])
                    .is_ok_and(|n| n.to_str().is_ok_and(|n| n == section_name))
            })
        }
        .expect("failed to get module info");

        // Merge all `LOAD` segments, as the PSP seems to only be able to handle one.
        // This code assumes all segments appear sequentially, and start at zero.
        {
            let load_segments = || {
                self.program_headers
                    .iter()
                    .filter(|ph| ph.p_type == PT_LOAD)
            };

            let start_vaddr = load_segments()
                .next()
                .expect("program had no LOAD segments")
                .p_vaddr;
            assert_eq!(start_vaddr, 0);

            let start_offset = load_segments().next().unwrap().p_offset;

            let mem_size = load_segments()
                .map(|ph| ph.p_offset + ph.p_memsz - start_offset)
                .max()
                .unwrap();

            let file_size = load_segments()
                .map(|ph| ph.p_offset + ph.p_filesz - start_offset)
                .max()
                .unwrap();

            let program_header = &mut self.program_headers[0];
            program_header.p_type = 1;
            program_header.p_vaddr = 0;
            program_header.p_paddr = {
                // Check if we are a kernel module.
                if module_info.sh_flags & 0x100 != 0 {
                    0x80000000 | module_info.sh_offset
                } else {
                    module_info.sh_offset
                }
            };
            program_header.p_filesz = file_size;
            program_header.p_memsz = mem_size;
            program_header.p_flags = 5;
            program_header.p_align = 0x10;
        }

        self
    }

    /// Write out the changes to a file.
    fn save(self, output: PathBuf) {
        let mut bytes = self.elf_bytes;

        // Write header to buffer.
        self.header
            .try_into_ctx(&mut bytes, Endian::Little)
            .expect("failed to write header");

        // Write updated relocations to buffer.
        for (i, rels) in self.relocations {
            let offset = self.section_headers[i].sh_offset as usize;
            for (j, rel) in rels.into_iter().enumerate() {
                let offset = offset + j * SIZEOF_REL;
                rel.try_into_ctx(&mut bytes[offset..], Endian::Little)
                    .expect("failed to write relocation");
            }
        }

        // Write section headers to buffer.
        for (i, section_header) in self.section_headers.into_iter().enumerate() {
            let offset = self.header.e_shoff as usize + i * self.header.e_shentsize as usize;
            section_header
                .try_into_ctx(&mut bytes[offset..], Endian::Little)
                .expect("failed to write section header");
        }

        // Write program headers to buffer.
        for (i, program_header) in self.program_headers.into_iter().enumerate() {
            let offset = self.header.e_phoff as usize + i * self.header.e_phentsize as usize;
            program_header
                .try_into_ctx(&mut bytes[offset..], Endian::Little)
                .expect("failed to write program headers");
        }

        fs::write(output, bytes).expect("failed to write file");
    }
}
