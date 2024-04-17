use clap::Parser;
use std::{fs, mem, path::PathBuf};

const SIGNATURE: [u8; 4] = *b"\0PBP";
const VERSION: u32 = 0x1_0000;

struct PbpHeader {
    signature: [u8; 4],
    version: u32,
    offsets: [u32; 8],
}

impl PbpHeader {
    fn to_bytes(&self) -> [u8; mem::size_of::<Self>()] {
        let mut bytes = [0; mem::size_of::<Self>()];

        bytes[0..4].copy_from_slice(&self.signature);
        bytes[4..8].copy_from_slice(&self.version.to_le_bytes());

        for (i, offset) in self.offsets.iter().enumerate() {
            let idx = i * 4 + 8;
            bytes[idx..idx + 4].copy_from_slice(&offset.to_le_bytes());
        }

        bytes
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "pack-pbp",
    author = "Marko Mijalkovic <marko.mijalkovic97@gmail.com>",
    version = "0.1",
    about = "Create Sony PSP packages"
)]
struct Args {
    #[arg(name = "output.pbp", help = "Output PBP file")]
    output: PathBuf,
    #[arg(name = "param.sfo", help = "Input PARAM.SFO file created with mksfo")]
    param: PathBuf,
    #[arg(name = "icon0.png", help = "Optional XMB icon")]
    icon0: PathBuf,
    #[arg(name = "icon1.pmf", help = "Optional animated XMB icon")]
    icon1: PathBuf,
    #[arg(
        name = "pic0.png",
        help = "Optional XMB background (overlayed on top of PIC1.PNG)"
    )]
    pic0: PathBuf,
    #[arg(name = "pic1.png", help = "Optional XMB background")]
    pic1: PathBuf,
    #[arg(
        name = "snd0.at3",
        help = "Optional XMB music (when present, sound from ICON1.PMF is ignored)"
    )]
    snd0: PathBuf,
    #[arg(name = "data.psp", help = "Executable file")]
    data_psp: PathBuf,
    #[arg(name = "data.psar", help = "Optional archive data")]
    data_psar: PathBuf,
}

fn main() {
    let args = Args::parse();

    let read = |value: PathBuf| {
        if value == PathBuf::from("NULL") {
            None
        } else {
            match fs::read(&value) {
                Ok(res) => Some(res),
                Err(err) => panic!("failed to read {}: {}", value.display(), err),
            }
        }
    };

    let output_path = args.output;

    let files = vec![
        read(args.param),
        read(args.icon0),
        read(args.icon1),
        read(args.pic0),
        read(args.pic1),
        read(args.snd0),
        read(args.data_psp),
        read(args.data_psar),
    ];

    let mut payload = Vec::new();
    let mut offsets = [0; 8];
    let mut current_offset = mem::size_of::<PbpHeader>() as u32;

    for (i, bytes) in files.into_iter().enumerate() {
        offsets[i] = current_offset;

        if let Some(bytes) = bytes {
            let len = bytes.len();
            payload.extend(bytes);
            current_offset += len as u32;
        }
    }

    let header = PbpHeader {
        signature: SIGNATURE,
        version: VERSION,
        offsets,
    };

    let mut output = Vec::new();
    output.extend(&header.to_bytes()[..]);
    output.extend(payload);

    if let Err(err) = fs::write(&output_path, output) {
        panic!("couldn't write to {}: {}", output_path.display(), err);
    }

    println!("Saved to {output_path:?}");
}
