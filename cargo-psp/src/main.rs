use std::{fs, env, io::ErrorKind, process::{self, Command, Stdio}};
use cargo_metadata::MetadataCommand;

const CONFIG_NAME: &str = "Psp.toml";

#[derive(serde_derive::Deserialize, Default)]
struct PspConfig {
    /// Title shown in the XMB menu.
    title: Option<String>,

    /// Path to 24bit 144x80 PNG icon shown in the XMB menu.
    xmb_icon_png: Option<String>,

    /// Path to animated icon shown in the XMB menu.
    ///
    /// The PSP expects a 29.97fps 144x80 PMF video file (custom Sony format).
    xmb_icon_pmf: Option<String>,

    /// Path to 24bit 480x272 PNG background shown in the XMB menu.
    xmb_background_png: Option<String>,

    /// Overlay background shown in the XMB menu.
    ///
    /// Exactly like `xmb_background_png`, but it is overlayed on top.
    xmb_background_overlay_png: Option<String>,

    /// Path to ATRAC3 audio file played in the XMB menu.
    xmb_music_at3: Option<String>,

    /// Path to associated PSAR data stored in the EBOOT.
    psar: Option<String>,
}

fn main() {
    let config = match fs::read(CONFIG_NAME) {
        Ok(bytes) => match toml::from_slice(&bytes) {
            Ok(config) => config,
            Err(e) => {
                println!("Failed to read Psp.toml: {}", e);
                println!("Please ensure that it is formatted correctly.");
                process::exit(1);
            }
        }

        Err(e) if e.kind() == ErrorKind::NotFound => PspConfig::default(),
        Err(e) => panic!("{}", e)
    };

    // Skip `cargo psp`
    let args = env::args().skip(2);

    let command = Command::new("xargo")
        .arg("build")
        .arg("--target")
        .arg("mipsel-sony-psp")
        .args(args)
        // TODO: merge with parent process value
        .env("RUSTFLAGS", "-C link-dead-code")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap_or_else(|e| {
            println!("Failed to run `xargo`: {}", e);
            println!(
                "Try running `cargo install xargo` and re-run this command"
            );

            process::exit(1);
        });

    if !command.status.success() {
        let code = match command.status.code() {
            Some(i) => i,
            None => 1,
        };

        process::exit(code);
    }

    let metadata = MetadataCommand::new()
        .exec()
        .expect("failed to get cargo metadata");

    // Is there a better way to do this?
    let profile_name = if env::args().any(|arg| arg == "--release") {
        "release"
    } else {
        "debug"
    };

    let bin_dir = metadata.target_directory
        .join("mipsel-sony-psp")
        .join(profile_name);

    for id in metadata.clone().workspace_members {
        let package = metadata[&id].clone();

        for target in package.targets {
            if target.kind.iter().any(|k| k == "bin") {
                let elf_path = bin_dir.join(&target.name);
                let prx_path = bin_dir.join(target.name.clone() + ".prx");

                let sfo_path = bin_dir.join("PARAM.SFO");
                let pbp_path = bin_dir.join("EBOOT.PBP");

                Command::new("prxgen")
                    .arg(&elf_path)
                    .arg(&prx_path)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()
                    .expect("failed to run prxgen");

                Command::new("mksfo")
                    .arg(config.title.clone().unwrap_or(target.name))
                    .arg(&sfo_path)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()
                    .expect("failed to run mksfo");

                Command::new("pack-pbp")
                    .arg(&pbp_path)
                    .arg(&sfo_path)
                    .arg(config.xmb_icon_png.clone().unwrap_or("NULL".into()))
                    .arg(config.xmb_icon_pmf.clone().unwrap_or("NULL".into()))
                    .arg(config.xmb_background_png.clone().unwrap_or("NULL".into()))
                    .arg(config.xmb_background_overlay_png.clone().unwrap_or("NULL".into()))
                    .arg(config.xmb_music_at3.clone().unwrap_or("NULL".into()))
                    .arg(&prx_path)
                    .arg(config.psar.clone().unwrap_or("NULL".into()))
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()
                    .expect("failed to run pack-pbp");
            }
        }
    }
}
