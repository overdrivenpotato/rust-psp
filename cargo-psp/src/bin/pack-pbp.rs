use clap::{App, AppSettings, Arg};
use std::{fs, mem};

const SIGNATURE: [u8; 4] = *b"\0PBP";
const VERSION: u32 = 0x1_0000;

#[derive(Clone, Copy)]
struct PbpHeader {
    signature: [u8; 4],
    version: u32,
    offsets: [u32; 8],
}

impl PbpHeader {
    fn to_bytes(self) -> [u8; mem::size_of::<Self>()] {
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

fn main() {
    let matches = App::new("pack-pbp")
        .version("0.1")
        .author("Marko Mijalkovic <marko.mijalkovic97@gmail.com>")
        .about("Create Sony PSP packages")
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("output.pbp")
                .takes_value(true)
                .help("Output PBP file")
                .required(true),
        )
        .arg(
            Arg::with_name("param.sfo")
                .takes_value(true)
                .help("Input PARAM.SFO file created with mksfo")
                .required(true),
        )
        .arg(
            Arg::with_name("icon0.png")
                .takes_value(true)
                .help("Optional XMB icon")
                .required(true),
        )
        .arg(
            Arg::with_name("icon1.pmf")
                .takes_value(true)
                .help("Optional animated XMB icon")
                .required(true),
        )
        .arg(
            Arg::with_name("pic0.png")
                .takes_value(true)
                .help("Optional XMB background (overlayed on top of PIC1.PNG)")
                .required(true),
        )
        .arg(
            Arg::with_name("pic1.png")
                .takes_value(true)
                .help("Optional XMB background")
                .required(true),
        )
        .arg(
            Arg::with_name("snd0.at3")
                .takes_value(true)
                .help("Optional XMB music (when present, sound from ICON1.PMF is ignored)")
                .required(true),
        )
        .arg(
            Arg::with_name("data.psp")
                .takes_value(true)
                .help("Executable file")
                .required(true),
        )
        .arg(
            Arg::with_name("data.psar")
                .takes_value(true)
                .help("Optional archive data")
                .required(true),
        )
        .get_matches();

    let read = |name: &str| {
        let value = matches.value_of(name).unwrap();

        if value == "NULL" {
            return None;
        }

        match fs::read(value) {
            Ok(b) => Some(b),
            Err(e) => panic!("failed to read {}: {}", value, e),
        }
    };

    let output_path = matches.value_of("output.pbp").unwrap();

    let files = vec![
        read("param.sfo"),
        read("icon0.png"),
        read("icon1.pmf"),
        read("pic0.png"),
        read("pic1.png"),
        read("snd0.at3"),
        read("data.psp"),
        read("data.psar"),
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

    if let Err(e) = fs::write(output_path, output) {
        panic!("couldn't write to {}: {}", output_path, e);
    }

    println!("Saved to {}", output_path);
}
