use cargo_metadata::{
    semver::{BuildMetadata, Prerelease},
    Message as CargoMessage, MetadataCommand,
};
use rustc_version::{Channel, Version};
use std::{
    collections::HashSet,
    env, fmt, fs,
    io::ErrorKind,
    process::{self, Command, Stdio},
};

mod fix_imports;

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
        let mut iter = date.split('-');

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

impl core::ops::Add<CommitDate> for CommitDate {
    type Output = CommitDate;

    fn add(self, rhs: CommitDate) -> Self::Output {
        Self {
            year: self.year + rhs.year,
            month: self.month + rhs.month,
            day: self.day + rhs.day,
        }
    }
}

// Minimum 2023-03-27, remember to update both commit date and version too,
// below. Note that the `day` field lags by one day, as the toolchain always
// contains the previous days' nightly rustc.
const MINIMUM_COMMIT_DATE: CommitDate = CommitDate {
    year: 2025,
    month: 3,
    day: 18,
};
const MINIMUM_RUSTC_VERSION: Version = Version {
    major: 1,
    minor: 87,
    patch: 0,
    pre: Prerelease::EMPTY,
    build: BuildMetadata::EMPTY,
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

    let old_version = MINIMUM_RUSTC_VERSION
        > Version {
            // Remove `-nightly` pre-release tag for comparison.
            pre: Prerelease::EMPTY,
            ..rustc_version.semver.clone()
        };

    let old_commit = match rustc_version.commit_date {
        None => false,
        Some(date) => {
            MINIMUM_COMMIT_DATE
                > CommitDate::parse(&date).expect("could not parse `rustc --version` commit date")
        }
    };

    if old_version || old_commit {
        println!(
            "cargo-psp requires rustc nightly version >= {}",
            MINIMUM_COMMIT_DATE
                + CommitDate {
                    year: 0,
                    month: 0,
                    day: 1
                },
        );
        println!("Please run `rustup update nightly` to upgrade your nightly version");

        process::exit(1);
    }

    let config = match fs::read_to_string(CONFIG_NAME) {
        Ok(value) => match toml::from_str(&value) {
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

    let build_std_flag = match env::var("RUST_PSP_BUILD_STD") {
        Ok(_) => {
            eprintln!("[NOTE]: Detected RUST_PSP_BUILD_STD env var, using \"build-std\".");
            "build-std"
        }
        Err(_) => "build-std=core,compiler_builtins,alloc,panic_unwind,panic_abort",
    };

    let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let mut build_process = Command::new(&cargo)
        .arg("build")
        .arg("-Z")
        .arg(build_std_flag)
        .arg("--target")
        .arg("mipsel-sony-psp")
        .arg("--message-format=json-render-diagnostics")
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let lone = {
        let output = Command::new(cargo)
            .arg("metadata")
            .arg("--format-version=1")
            .arg("-Z")
            .arg(build_std_flag)
            .stderr(Stdio::inherit())
            .output()
            .unwrap();

        if !output.status.success() {
            panic!(
                "`cargo metadata` command exited with status: {:?}",
                output.status
            );
        }

        let metadata = MetadataCommand::parse(
            std::str::from_utf8(&output.stdout)
                .expect("`cargo metadata` command returned non UTF-8 bytes"),
        )
        .expect("failed to parse `cargo metadata` command's stdout");

        let workspace_members: HashSet<_> = metadata.workspace_members.iter().collect();
        let total_executables = metadata
            .packages
            .iter()
            .filter(|p| workspace_members.contains(&p.id))
            .flat_map(|p| &p.targets)
            .filter(|t| t.crate_types.iter().any(|ct| *ct == "bin".into()))
            .count();

        total_executables == 1
    };

    let reader = std::io::BufReader::new(build_process.stdout.take().unwrap());
    let built_executables: Vec<_> = CargoMessage::parse_stream(reader)
        .flat_map(|msg| match msg.unwrap() {
            CargoMessage::CompilerArtifact(art) => art.executable,
            _ => None,
        })
        .collect();

    let status = build_process.wait().unwrap();
    if !status.success() {
        eprintln!("`cargo build` command exited with status: {:?}", status);
        process::exit(status.code().unwrap_or(1));
    }

    // TODO: Error if no bin is ever found.
    for elf_path in built_executables {
        let prx_path = elf_path.with_extension("prx");

        let [sfo_path, pbp_path] = ["PARAM.SFO", "EBOOT.PBP"].map(|e| {
            if lone {
                elf_path.with_file_name(e)
            } else {
                elf_path.with_extension(e)
            }
        });

        fix_imports::fix(&elf_path);

        let status = Command::new("prxgen")
            .arg(&elf_path)
            .arg(&prx_path)
            .status()
            .expect("failed to run prxgen");

        assert!(status.success(), "prxgen failed: {}", status);

        let config_args = vec![
            ("-s", "DISC_ID", config.disc_id.clone()),
            ("-s", "DISC_VERSION", config.disc_version.clone()),
            ("-s", "LANGUAGE", config.language.clone()),
            (
                "-d",
                "PARENTAL_LEVEL",
                config.parental_level.as_ref().map(u32::to_string),
            ),
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

        let status = Command::new("mksfo")
            // Add the optional config args
            .args({
                config_args
                    .into_iter()
                    // Filter through all the values that are not `None`
                    .filter_map(|(f, k, v)| v.map(|v| (f, k, v)))
                    // Map into 2 arguments, e.g. "-s" "NAME=VALUE"
                    .flat_map(|(flag, key, value)| vec![flag.into(), format!("{}={}", key, value)])
            })
            .arg(
                config
                    .title
                    .as_ref()
                    .map(|s| s.as_ref())
                    .or_else(|| elf_path.file_stem())
                    .unwrap(),
            )
            .arg(&sfo_path)
            .status()
            .expect("failed to run mksfo");

        assert!(status.success(), "mksfo failed: {}", status);

        let status = Command::new("pack-pbp")
            .arg(&pbp_path)
            .arg(&sfo_path)
            .arg(config.xmb_icon_png.as_deref().unwrap_or("NULL"))
            .arg(config.xmb_icon_pmf.as_deref().unwrap_or("NULL"))
            .arg(
                config
                    .xmb_background_overlay_png
                    .as_deref()
                    .unwrap_or("NULL"),
            )
            .arg(config.xmb_background_png.as_deref().unwrap_or("NULL"))
            .arg(config.xmb_music_at3.as_deref().unwrap_or("NULL"))
            .arg(&prx_path)
            .arg(config.psar.as_deref().unwrap_or("NULL"))
            .status()
            .expect("failed to run pack-pbp");

        assert!(status.success(), "pack-pbp failed: {}", status);
    }
}
