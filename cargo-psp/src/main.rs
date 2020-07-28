use cargo_metadata::MetadataCommand;
use rustc_version::{Version, Channel};
use std::{
    env, fs, fmt,
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

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug)]
struct CommitDate {
    year: i32,
    month: i32,
    day: i32,
}

impl CommitDate {
    fn parse(date: &str) -> Option<Self> {
        let mut iter = date.split("-");

        let year = iter.next()?.parse().ok()?;
        let month = iter.next()?.parse().ok()?;
        let day = iter.next()?.parse().ok()?;

        Some(Self { year, month, day })
    }
}

impl fmt::Display for CommitDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

const MINIMUM_COMMIT_DATE: CommitDate = CommitDate { year: 2020, month: 06, day: 04 };
const MINIMUM_RUSTC_VERSION: Version = Version {
    major: 1,
    minor: 45,
    patch: 0,
    pre: Vec::new(),
    build: Vec::new(),
};

fn main() {
    let rustc_version = rustc_version::version_meta().unwrap();

    if rustc_version.channel > Channel::Nightly {
        println!("cargo-psp requires a nightly rustc version.");
        println!(
            "Please run `rustup override set nightly` to use nightly in the \
            current directory."
        );
        process::exit(1);
    }

    let old_version = MINIMUM_RUSTC_VERSION > Version {
        // Remove `-nightly` pre-release tag for comparison.
        pre: Vec::new(),
        ..rustc_version.semver.clone()
    };

    let old_commit = match rustc_version.commit_date {
        None => false,
        Some(date) => MINIMUM_COMMIT_DATE > CommitDate::parse(&date)
            .expect("could not parse `rustc --version` commit date"),
    };

    if old_version || old_commit {
        println!(
            "cargo-psp requires rustc nightly version >= {}",
            MINIMUM_COMMIT_DATE,
        );
        println!(
            "Please run `rustup update nightly` to upgrade your nightly version"
        );

        process::exit(1);
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

    // FIXME: This is a workaround. This should eventually be removed.
    let rustflags = env::var("RUSTFLAGS").unwrap_or("".into())
        + " -C link-dead-code -C opt-level=3";

    let mut process = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("mipsel-sony-psp")
        .arg("-Z")
        .arg("build-std")
        .args(args)
        .env("RUSTFLAGS", rustflags)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

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
