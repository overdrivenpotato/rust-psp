rustup set profile minimal
rustup update --no-self-update %RUSTUP_TOOLCHAIN%

rustup component add rust-src

# Test formatting
rustup component add rustfmt
cd repo/
cargo fmt --check
cd ..

# build cargo-psp
cd repo/cargo-psp/
cargo build
cd ../..

# build the test project
cd repo/ci/tests
cargo psp
cd ../../..

# Make the output directory, in case it is not specified as an output for this
# concourse task.
mkdir  rust-build-dir

copy repo/ci/tests/target/mipsel-sony-psp/debug/* rust-build-dir
