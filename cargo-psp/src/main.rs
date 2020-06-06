use std::{fs, env, io::ErrorKind, path::Path, process::{self, Command, Stdio}};
use cargo_metadata::MetadataCommand;

const CONFIG_NAME: &str = "Psp.toml";

#[derive(serde_derive::Deserialize, Default)]
struct PspConfig {
    title: Option<String>,

    // TODO: Other parameters
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

        Err(e) if e.kind() == ErrorKind::NotFound => Default::default(),
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
        .expect("failed to run xargo");

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
                let prx_path = bin_dir.join(target.name + ".prx");

                let sfo_path = bin_dir.join("PARAM.SFO");
                let pbp_path = bin_dir.join("EBOOT.PBP");

                prxgen(&elf_path, &prx_path);
                mksfo(&sfo_path, &config);
                pack_pbp(&prx_path, &sfo_path, &pbp_path, &config);
            }
        }
    }
}

fn prxgen<P: AsRef<Path>>(elf: P, prx: P) {
    Command::new("prxgen")
        .arg(elf.as_ref().as_os_str())
        .arg(prx.as_ref().as_os_str())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to run prxgen");
}

fn mksfo<P: AsRef<Path>>(out: P, config: &PspConfig) {
    Command::new("mksfo")
        .arg(config.title.clone().unwrap_or("Default Title: cargo-psp".into()))
        .arg(out.as_ref().as_os_str())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to run mksfo");
}

fn pack_pbp<P: AsRef<Path>>(prx: P, sfo: P, out: P, config: &PspConfig) {
    Command::new("pack-pbp")
        .arg(out.as_ref().as_os_str())
        .arg(sfo.as_ref().as_os_str())
        .arg("NULL")
        .arg("NULL")
        .arg("NULL")
        .arg("NULL")
        .arg("NULL")
        .arg(prx.as_ref().as_os_str())
        .arg("NULL")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to run pack-pbp");
}
