use std::{env, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // TODO: Do we even need to copy the library over? Maybe we can just link
    // directly from the current directory.
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_file = Path::new(&out_dir).join("libunwind.a");
    std::fs::copy("./libunwind.a", out_file).unwrap();

    println!("cargo:rustc-link-lib=static=unwind");
    println!("cargo:rustc-link-search=native={}", out_dir);
}
