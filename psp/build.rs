use std::{env, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=libunwind.a");
    println!("cargo:rerun-if-env-changed=RUSTFLAGS");

    if env::var("CARGO_FEATURE_STUB_ONLY").is_ok() {
        return;
    }

    // Figure out whether to use the LTO libunwind, or the regular one.
    let libunwind = if env::var("CARGO_ENCODED_RUSTFLAGS")
        .unwrap()
        .split('\x1f')
        .any(|flags| flags.starts_with("-Clinker-plugin-lto"))
    {
        "./libunwind_lto.a"
    } else {
        "./libunwind.a"
    };

    // TODO: Do we even need to copy the library over? Maybe we can just link
    // directly from the current directory.
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_file = Path::new(&out_dir).join("libunwind.a");
    std::fs::copy(libunwind, out_file).unwrap();

    println!("cargo:rustc-link-lib=static=unwind");
    println!("cargo:rustc-link-search=native={}", out_dir);
}
