use cargo_metadata::MetadataCommand;
use std::{
    env, fs,
    io::ErrorKind,
    process::{self, Command, Stdio},
};

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

    /// Setting this to 1 indicates the game should be autolaunched at bootup.
    bootable: Option<u32>,

    //todo enum?
    /// One of the following:
    /// - WG (WLAN Game)
    /// - MS (MemoryStick Save)
    /// - MG (MemoryStick Game)
    /// - UG (UMD Game)
    category: Option<String>,

    /// Product number of the game, e.g. ABCD-12345.
    disc_id: Option<String>,

    /// Which disc (out of disc_total) is this? Starts at 1.
    disc_number: Option<u32>,

    /// Total number of UMD discs for the game.
    disc_total: Option<u32>,

    /// Version of the game, e.g. "1.00".
    disc_version: Option<String>,

    /// Unknown.
    driver_path: Option<String>,

    // todo enum
    /// Language of the game.
    language: Option<String>,

    // todo enum
    /// Parental Control level needed to access the file. 1-11
    /// - 1 - General audience
    /// - 5 - 12 year old
    /// - 7 - 15 year old
    /// - 9 - 18 year old
    parental_level: Option<u32>,

    /// PSP Firmware Version required by the game (e.g. "6.61").
    psp_system_ver: Option<String>,

    // todo document values
    /// Bitmask of allowed regions, 0x8000 is region 2?
    region: Option<u32>,

    /// Text shown under the details heading in the save game menu.
    savedata_detail: Option<String>,

    /// The name of the subdirectory where this game stores it's save files.
    /// e.g. ("UCJS10001")
    savedata_directory: Option<String>,

    /// Text shown under the "Saved Data" heading of the save game menu.
    savedata_title: Option<String>,

    /// Japanese localized title.
    title_jp: Option<String>,

    /// French localized title.
    title_fr: Option<String>,

    /// Spanish localized title.
    title_es: Option<String>,

    /// German localized title.
    title_de: Option<String>,

    /// Italian localized title.
    title_it: Option<String>,

    /// Dutch localized title.
    title_nl: Option<String>,

    /// Portugese localized title.
    title_pt: Option<String>,

    /// Russian localized title.
    title_ru: Option<String>,

    /// Used by the firmware updater to denote the firmware version it updates to.
    updater_version: Option<String>,
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
        },

        Err(e) if e.kind() == ErrorKind::NotFound => PspConfig::default(),
        Err(e) => panic!("{}", e),
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
            println!("Try running `cargo install xargo` and re-run this command");

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

    let bin_dir = metadata
        .target_directory
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

                let mut mksfo_args: Vec<String> = Vec::new();
                mksfo_args.push(config.title.clone().unwrap_or(target.name));

                // TODO this is pretty janky
                mksfo_args.push(sfo_path.to_str().unwrap().to_string());

                if config.bootable.is_some() {
                    mksfo_args.push(format!("-d BOOTABLE={}", config.bootable.unwrap()));
                }
                if config.category.is_some() {
                    mksfo_args.push(format!("-s CATEGORY={}", config.category.clone().unwrap()));
                }
                if config.disc_id.is_some() {
                    mksfo_args.push(format!("-s DISC_ID={}", config.disc_id.clone().unwrap()));
                }
                if config.disc_number.is_some() {
                    mksfo_args.push(format!("-d DISC_NUMBER={}", config.disc_number.unwrap()));
                }
                if config.disc_total.is_some() {
                    mksfo_args.push(format!("-d DISC_TOTAL={}", config.disc_total.unwrap()));
                }
                if config.disc_version.is_some() {
                    mksfo_args.push(format!(
                        "-s DISC_VERSION={}",
                        config.disc_version.clone().unwrap()
                    ));
                }
                if config.driver_path.is_some() {
                    mksfo_args.push(format!(
                        "-s DRIVER_PATH={}",
                        config.driver_path.clone().unwrap()
                    ));
                }
                if config.language.is_some() {
                    mksfo_args.push(format!("-s LANGUAGE={}", config.language.clone().unwrap()));
                }
                if config.parental_level.is_some() {
                    mksfo_args.push(format!(
                        "-d PARENTAL_LEVEL={}",
                        config.parental_level.unwrap()
                    ));
                }
                if config.psp_system_ver.is_some() {
                    mksfo_args.push(format!(
                        "-s PSP_SYSTEM_VER={}",
                        config.psp_system_ver.clone().unwrap()
                    ));
                }
                if config.region.is_some() {
                    mksfo_args.push(format!("-d REGION={}", config.region.unwrap()));
                }
                if config.savedata_detail.is_some() {
                    mksfo_args.push(format!(
                        "-s SAVEDATA_DETAIL={}",
                        config.savedata_detail.clone().unwrap()
                    ));
                }
                if config.savedata_directory.is_some() {
                    mksfo_args.push(format!(
                        "-s SAVEDATA_DIRECTORY={}",
                        config.savedata_directory.clone().unwrap()
                    ));
                }
                if config.savedata_title.is_some() {
                    mksfo_args.push(format!(
                        "-s SAVEDATA_TITLE={}",
                        config.savedata_title.clone().unwrap()
                    ));
                }
                if config.title_jp.is_some() {
                    mksfo_args.push(format!("-s TITLE_0={}", config.title_jp.clone().unwrap()));
                }
                if config.title_fr.is_some() {
                    mksfo_args.push(format!("-s TITLE_2={}", config.title_fr.clone().unwrap()));
                }
                if config.title_es.is_some() {
                    mksfo_args.push(format!("-s TITLE_3={}", config.title_es.clone().unwrap()));
                }
                if config.title_de.is_some() {
                    mksfo_args.push(format!("-s TITLE_4={}", config.title_de.clone().unwrap()));
                }
                if config.title_it.is_some() {
                    mksfo_args.push(format!("-s TITLE_5={}", config.title_it.clone().unwrap()));
                }
                if config.title_nl.is_some() {
                    mksfo_args.push(format!("-s TITLE_6={}", config.title_nl.clone().unwrap()));
                }
                if config.title_pt.is_some() {
                    mksfo_args.push(format!("-s TITLE_7={}", config.title_pt.clone().unwrap()));
                }
                if config.title_ru.is_some() {
                    mksfo_args.push(format!("-s TITLE_8={}", config.title_ru.clone().unwrap()));
                }
                if config.updater_version.is_some() {
                    mksfo_args.push(format!(
                        "-s UPDATER_VER={}",
                        config.updater_version.clone().unwrap()
                    ));
                }

                Command::new("mksfo")
                    .args(mksfo_args.as_slice())
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
                    .arg(
                        config
                            .xmb_background_overlay_png
                            .clone()
                            .unwrap_or("NULL".into()),
                    )
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
