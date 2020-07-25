use cargo_metadata::MetadataCommand;
use std::{
    env, fs, thread,
    io::{self, ErrorKind, Read, Write},
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
    ///
    /// Must be 66kbps, under 500KB and under 55 seconds.
    xmb_music_at3: Option<String>,

    /// Path to associated PSAR data stored in the EBOOT.
    psar: Option<String>,

    /// Product number of the game, in the format `ABCD-12345`.
    ///
    /// Example: UCJS-10001
    disc_id: Option<String>,

    /// Version of the game, e.g. "1.00".
    disc_version: Option<String>,

    // TODO: enum
    /// Language of the game. "JP" indicates Japanese, even though this is not
    /// the proper ISO 639 code...
    language: Option<String>,

    // TODO: enum
    /// Parental Control level needed to access the file. 1-11
    /// - 1 = General audience
    /// - 5 = 12 year old
    /// - 7 = 15 year old
    /// - 9 = 18 year old
    parental_level: Option<u32>,

    /// PSP Firmware Version required by the game (e.g. "6.61").
    psp_system_ver: Option<String>,

    // TODO: document values
    /// Bitmask of allowed regions. (0x8000 is region 2?)
    region: Option<u32>,

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

const SUBPROCESS_ENV_VAR: &str = "__CARGO_PSP_RUN_XARGO";

fn main() {
    if env::var(SUBPROCESS_ENV_VAR).is_ok() {
        return xargo::main_inner(xargo::XargoMode::Build);
    }

    // Ensure there is no `Xargo.toml` file already.
    match fs::metadata("Xargo.toml") {
        Err(e) if e.kind() == ErrorKind::NotFound => {},
        Err(e) => panic!("{}", e),
        Ok(_) => {
            println!("Found Xargo.toml file.");
            println!("Please remove this to coninue, as it interferes with `cargo-psp`.");

            process::exit(1);
        }
    }

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

    let libc_directive = match env::var("CARGO_PSP_LIBC") {
        Ok(loc) => format!(
            r#"[patch.crates-io.libc]
            path = "{}"
            "#, loc),
        _ => "".into()
    };


    let xargo_toml = format!(r#"
[target.mipsel-sony-psp.dependencies.core]
stage = 0

[target.mipsel-sony-psp.dependencies.alloc]
stage = 1

[target.mipsel-sony-psp.dependencies.panic_unwind]
stage = 2

[target.mipsel-sony-psp.dependencies.std]
stage = 3
{libc_directive}
"#, libc_directive=libc_directive);


    fs::write("Xargo.toml", xargo_toml).unwrap();

    // FIXME: This is a workaround. This should eventually be removed.
    let rustflags = env::var("RUSTFLAGS").unwrap_or("".into())
        + " -C link-dead-code -C opt-level=3";

    let mut process = Command::new("cargo-psp")
        // Relaunch as xargo wrapper.
        .env(SUBPROCESS_ENV_VAR, "1")
        .arg("build")
        .arg("--target")
        .arg("mipsel-sony-psp")
        .args(args)
        .env("RUSTFLAGS", rustflags)
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let xargo_stderr = process.stderr.take();

    // This is a pretty big hack. We wait until `xargo` starts printing and then
    // remove the toml. Then we have to manually pipe the output to our stdout.
    //
    // Ideally we could just set `XARGO_TOML_PATH` to some temporary file.
    thread::spawn(move || {
        let mut xargo_stderr = xargo_stderr.unwrap();
        let mut stderr = io::stderr();
        let mut removed_xargo_toml = false;
        let mut buf = vec![0; 8192];

        loop {
            let bytes = xargo_stderr.read(&mut buf).unwrap();

            if !removed_xargo_toml {
                fs::remove_file("Xargo.toml").unwrap();
                removed_xargo_toml = true;
            }

            if bytes == 0 {
                break
            }

            stderr.write_all(&buf[0..bytes]).unwrap();
        }
    });


    let status = process.wait().unwrap();

    if !status.success() {
        let code = match status.code() {
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

                let config_args = vec![
                    ("-s", "DISC_ID", config.disc_id.clone()),
                    ("-s", "DISC_VERSION", config.disc_version.clone()),
                    ("-s", "LANGUAGE", config.language.clone()),
                    ("-d", "PARENTAL_LEVEL", config.parental_level.as_ref().map(u32::to_string)),
                    ("-s", "PSP_SYSTEM_VER", config.psp_system_ver.clone()),
                    ("-d", "REGION", config.region.as_ref().map(u32::to_string)),
                    ("-s", "TITLE_0", config.title_jp.clone()),
                    ("-s", "TITLE_2", config.title_fr.clone()),
                    ("-s", "TITLE_3", config.title_es.clone()),
                    ("-s", "TITLE_4", config.title_de.clone()),
                    ("-s", "TITLE_5", config.title_it.clone()),
                    ("-s", "TITLE_6", config.title_nl.clone()),
                    ("-s", "TITLE_7", config.title_pt.clone()),
                    ("-s", "TITLE_8", config.title_ru.clone()),
                    ("-s", "UPDATER_VER", config.updater_version.clone()),
                ];

                Command::new("mksfo")
                    // Add the optional config args
                    .args({
                        config_args
                            .into_iter()

                            // Filter through all the values that are not `None`
                            .filter_map(|(f, k, v)| v.map(|v| (f, k, v)))

                            // Map into 2 arguments, e.g. "-s" "NAME=VALUE"
                            .flat_map(|(flag, key, value)| vec![
                                flag.into(),
                                format!("{}={}", key, value),
                            ])
                    })
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
