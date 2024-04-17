use clap::Parser;
use goblin::{
    container::{Container, Ctx, Endian},
    elf::{
        program_header::{PF_R, PF_W, PF_X, PT_LOAD},
        section_header::{SHT_NOBITS, SHT_NULL},
        Elf, Header, ProgramHeader, SectionHeader,
    },
    elf32,
};
use scroll::ctx::{IntoCtx, TryIntoCtx};
use std::{
    borrow::Cow,
    fs::{self, File},
    io::Write,
    iter,
    path::PathBuf,
};

const SHT_PRXREL: u32 = 0x7000_00A0;
const MODULE_INFO_SECTION: &str = ".rodata.sceModuleInfo";
const DATA_OFFSET: usize = elf32::header::SIZEOF_EHDR + elf32::program_header::SIZEOF_PHDR;

#[derive(Debug, Clone)]
struct Section<'a> {
    header: Cow<'a, SectionHeader>,
    data: Cow<'a, [u8]>,
    name: Cow<'a, str>,
}

#[derive(Parser, Debug)]
#[command(
    name = "prxmin",
    author = "Marko Mijalkovic <marko.mijalkovic97@gmail.com>",
    version = "0.1",
    about = "Minify (and strip) PRX files"
)]
struct Args {
    #[arg(name = "module.prx", help = "PRX module to minify")]
    input: PathBuf,
    #[arg(
        name = "minified.prx.min",
        help = "Optional output file name (default adds .min)"
    )]
    output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let bin = fs::read(&args.input).unwrap();
    let elf = Elf::parse(&bin).unwrap();

    let shstrtab = {
        let idx = elf.header.e_shstrndx as usize;
        let start = elf.section_headers[idx].sh_offset as usize;
        let size = elf.section_headers[idx].sh_size as usize;
        &bin[start..start + size]
    };

    let sections = elf
        .section_headers
        .iter()
        .map(|header| Section {
            header: Cow::Borrowed(header),
            data: {
                let start = header.sh_offset as usize;
                let size = header.sh_size as usize;
                Cow::Borrowed(&bin[start..start + size])
            },
            name: {
                let end = &shstrtab[header.sh_name..]
                    .iter()
                    .position(|&b| b == 0)
                    .unwrap();

                let bytes = &shstrtab[header.sh_name..header.sh_name + end];

                Cow::Borrowed(std::str::from_utf8(bytes).unwrap())
            },
        })
        .collect::<Vec<_>>();

    let null = &sections[0];

    let allocated = sections
        .iter()
        .filter(|s| s.header.is_alloc())
        .filter(|s| s.header.sh_type != SHT_NOBITS);

    let relocations = sections
        .iter()
        .filter(|s| s.header.sh_type == SHT_PRXREL)
        .filter(|s| sections[s.header.sh_info as usize].header.is_alloc());

    let nobits = sections.iter().filter(|s| s.header.sh_type == SHT_NOBITS);

    let mut shstrtab = Section {
        data: Cow::Owned(Vec::new()),
        ..sections[elf.header.e_shstrndx as usize].clone()
    };

    let mut new_sections = Some(null)
        .into_iter()
        .chain(allocated)
        .chain(nobits)
        .chain(relocations)
        .cloned()
        .collect::<Vec<_>>();

    for section in &mut new_sections {
        section.header.to_mut().sh_name = shstrtab.data.len();

        // Add name and null byte.
        shstrtab.data.to_mut().extend(section.name.as_bytes());
        shstrtab.data.to_mut().extend(&[0]);
    }

    // Add shstrtab to itself.
    shstrtab.header.to_mut().sh_name = shstrtab.data.len();
    shstrtab.data.to_mut().extend(shstrtab.name.as_bytes());
    shstrtab.data.to_mut().extend(&[0]);
    shstrtab.header.to_mut().sh_size = shstrtab.data.len() as u64;

    new_sections.push(shstrtab);

    let names = new_sections
        .iter()
        .map(|s| s.name.clone())
        .collect::<Vec<_>>();

    // Fix relocation sh_info index.
    for s in new_sections
        .iter_mut()
        .filter(|s| s.header.sh_type == SHT_PRXREL)
    {
        let old_idx = s.header.sh_info as usize;
        let idx = names
            .iter()
            .position(|n| *n == sections[old_idx].name)
            .unwrap();

        s.header.to_mut().sh_info = idx as u32;
        s.header.to_mut().sh_link = 0;
    }

    let mut body = Vec::<u8>::new();

    for section in &mut new_sections {
        if section.header.sh_type == SHT_NULL {
            continue;
        }

        let align = section.header.sh_addralign as usize;

        if align > 0 {
            // Add padding.
            let padding = (align - ((body.len() + DATA_OFFSET) & (align - 1))) & (align - 1);
            let padding = iter::repeat(0).take(padding);
            body.extend(padding);
        }

        section.header.to_mut().sh_offset = (DATA_OFFSET + body.len()) as u64;

        if section.header.sh_type == SHT_NOBITS {
            continue;
        }

        // Fill in the actual bytes.
        body.extend(section.data.as_ref());
    }

    let text_start = new_sections[1].header.sh_offset;

    let program_header = ProgramHeader {
        p_type: PT_LOAD,
        p_flags: PF_R | PF_W | PF_X,
        p_vaddr: 0,

        p_paddr: new_sections
            .iter()
            .find(|s| s.name == MODULE_INFO_SECTION)
            .map(|s| s.header.sh_offset)
            .unwrap(),

        p_offset: new_sections[1].header.sh_offset,

        p_filesz: new_sections
            .iter()
            .rev()
            .find(|s| s.header.sh_type != SHT_NOBITS && s.header.is_alloc())
            .map(|s| s.header.sh_offset + s.header.sh_size - text_start)
            .unwrap(),

        p_memsz: new_sections
            .iter()
            .rev()
            .find(|s| s.header.sh_type == SHT_NOBITS)
            .map(|s| s.header.sh_offset + s.header.sh_size - text_start)
            .unwrap(),

        p_align: new_sections
            .iter()
            .map(|s| s.header.sh_addralign)
            .max()
            .unwrap(),
    };

    let header = Header {
        e_shstrndx: new_sections.len() as u16 - 1,
        e_phoff: elf32::header::SIZEOF_EHDR as u64,
        e_shoff: (DATA_OFFSET + body.len()) as u64,
        e_phnum: 1,
        e_shnum: new_sections.len() as u16,
        ..elf.header
    };

    let ctx = Ctx {
        container: Container::Little,
        le: Endian::Little,
    };

    let out_file = args.output.unwrap_or(args.input.with_extension(".prx.min"));
    let mut out = File::create(out_file).unwrap();

    let mut buf = vec![0; elf32::header::SIZEOF_EHDR];
    header.into_ctx(&mut buf, ctx);
    out.write_all(&buf).unwrap();

    let mut buf = vec![0; elf32::program_header::SIZEOF_PHDR];
    program_header.try_into_ctx(&mut buf, ctx).unwrap();
    out.write_all(&buf).unwrap();

    out.write_all(&body).unwrap();

    let mut buf = vec![0; elf32::section_header::SIZEOF_SHDR];

    for section in new_sections {
        section.header.into_owned().into_ctx(&mut buf, ctx);
        out.write_all(&buf).unwrap();
    }
}
