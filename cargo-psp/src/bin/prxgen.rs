use std::{fs, path::Path, collections::HashMap};
use goblin::elf32::{
    header::Header, reloc::Rel,
    section_header::{SectionHeader, SHT_REL, SHF_ALLOC},
    program_header::{ProgramHeader, PT_LOAD},
};
use scroll::{Endian, ctx::{TryIntoCtx, TryFromCtx}};
use clap::{App, Arg, AppSettings};

const PRX_ELF_TYPE: u16 = 0xffa0;
const PRX_SHT_REL: u32 = 0x700000A0;

fn main() {
    let matches = App::new("prxgen")
        .version("0.1")
        .author("Marko Mijalkovic <marko.mijalkovic97@gmail.com>")
        .about("Convert PSP ELF files to PRX format")
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("in_file.elf")
                .takes_value(true)
                .help("Input ELF file")
                .required(true)
        )
        .arg(
            Arg::with_name("out_file.prx")
                .takes_value(true)
                .help("Output PRX file")
                .required(true)
        )
        .arg(
            Arg::with_name("minfo")
                .long("minfo")
                .takes_value(true)
                .default_value(".rodata.sceModuleInfo")
                .help("Alternative name for .rodata.sceModuleInfo section")
        )
        .get_matches();

    let in_file = matches.value_of("in_file.elf").unwrap();
    let mod_info_sh_name = matches.value_of("minfo").unwrap();

    let mut prx_gen = PrxGen::load(in_file, mod_info_sh_name);
    prx_gen.modify();
    prx_gen.save(matches.value_of("out_file.prx").unwrap());
}

struct PrxGen<'a> {
    mod_info_sh_name: &'a str,

    elf_bytes: Vec<u8>,

    header: Header,
    section_headers: Vec<SectionHeader>,
    program_headers: Vec<ProgramHeader>,

    // Section index -> Vec<Rel>
    relocations: HashMap<usize, Vec<Rel>>,
}

impl<'a> PrxGen<'a> {
    /// Load the input ELF file and parse important structures.
    fn load<P: AsRef<Path>>(path: P, mod_info_sh_name: &'a str) -> Self {
        let bytes = fs::read(path).unwrap();
        let header = Header::parse(&bytes).unwrap();
        let section_headers = SectionHeader::from_bytes(
            &bytes[header.e_shoff as usize..],
            header.e_shnum as usize,
        );
        let program_headers = ProgramHeader::from_bytes(
            &bytes[header.e_phoff as usize..],
            header.e_phnum as usize,
        );

        let relocations = section_headers.iter()
            .enumerate()
            .filter(|(_, sh)| sh.sh_type == SHT_REL)
            .map(|(i, sh)| {
                let start_idx = sh.sh_offset as usize;
                let end_idx = sh.sh_size as usize + start_idx;

                let relocs = (&bytes[start_idx..end_idx])
                    .chunks(8)
                    .map(|rel_bytes| {
                        Rel::try_from_ctx(rel_bytes, Endian::Little).unwrap().0
                    })
                    .collect();

                (i, relocs)
            })
            .collect();

        Self {
            mod_info_sh_name,
            elf_bytes: bytes,
            header,
            section_headers,
            program_headers,
            relocations,
        }
    }

    /// Modify the inner structures to create a PRX format file.
    fn modify(&mut self) {
        // Change ELF type
        self.header.e_type = PRX_ELF_TYPE;

        // Immutable copy for indexing.
        let section_headers = self.section_headers.clone();

        // Change relocation section types
        for section_header in &mut self.section_headers {
            if section_header.sh_type == SHT_REL {
                let sh_target = section_headers[section_header.sh_info as usize];

                if sh_target.sh_flags & SHF_ALLOC != 0 {
                    section_header.sh_type = PRX_SHT_REL;
                }
            }
        }

        // Change all relocation types.
        for (i, rels) in &mut self.relocations {
            if self.section_headers[*i].sh_type == PRX_SHT_REL {
                for rel in rels {
                    // Set upper 24 bits to 0 (OFS_BASE, ADDR_BASE).
                    rel.r_info &= 0xff;
                }
            }
        }

        // Change first program header physical address to module info file offset
        // TODO: Kernel mode support
        self.program_headers[0].p_paddr = {
            // Section header string table
            let sh_string_table = self.section_headers[self.header.e_shstrndx as usize];

            let start_idx = sh_string_table.sh_offset as usize;
            let end_idx = start_idx + sh_string_table.sh_size as usize;

            let section_names = &self.elf_bytes[start_idx..end_idx];

            self.section_headers
                .iter()
                .find_map(|sh| {
                    let name = &section_names[sh.sh_name as usize..]
                        .split(|b| *b == 0)
                        .next()
                        .map(Vec::from)
                        .map(String::from_utf8)
                        // All section header names should be utf8 or something is
                        // severely wrong.
                        .map(Result::unwrap)
                        .unwrap();

                    if name == self.mod_info_sh_name {
                        Some(sh.sh_offset)
                    } else {
                        None
                    }
                })
                .unwrap()
        };

        // Merge all segments. The PSP seems to only be able to handle 1 `LOAD`
        // segment. This code assumes that all load segments appear sequentially
        // and that the first segment is loaded at virtual address 0. Assertions
        // ensure this is the case.
        {
            let load_segments = || self.program_headers.iter()
                .filter(|ph| ph.p_type == PT_LOAD);

            // First segment needs to be loaded to 0.
            assert_eq!(0, load_segments().next().unwrap().p_vaddr);

            let start_offset = load_segments().next().unwrap().p_offset;

            let mem_size = load_segments()
                .map(|ph| ph.p_offset + ph.p_memsz - start_offset)
                .max()
                .unwrap();

            let file_size = load_segments()
                .map(|ph| ph.p_offset + ph.p_filesz - start_offset)
                .max()
                .unwrap();

            self.program_headers[0].p_filesz = file_size;
            self.program_headers[0].p_memsz = mem_size;

            self.header.e_phnum = 1;
        }
    }

    /// Write out the changes to a file.
    fn save<P: AsRef<Path>>(self, output: P) {
        let mut bytes = self.elf_bytes;

        // Write header to buffer
        self.header.try_into_ctx(&mut bytes, Endian::Little).unwrap();

        // Write relocations to buffer
        for (i, rels) in self.relocations {
            let offset = self.section_headers[i].sh_offset as usize;

            for (j, rel) in rels.into_iter().enumerate() {
                rel.try_into_ctx(&mut bytes[offset + j * 8..], Endian::Little).unwrap();
            }
        }

        // Write section headers to buffer
        for (i, section_header) in self.section_headers.into_iter().enumerate() {
            let offset = self.header.e_shoff as usize + i * self.header.e_shentsize as usize;
            section_header.try_into_ctx(&mut bytes[offset..], Endian::Little).unwrap();
        }

        // Write program headers to buffer
        for (i, program_header) in self.program_headers.into_iter().enumerate() {
            let offset = self.header.e_phoff as usize + i * self.header.e_phentsize as usize;
            program_header.try_into_ctx(&mut bytes[offset..], Endian::Little).unwrap();
        }

        fs::write(output, bytes).unwrap();
    }
}
