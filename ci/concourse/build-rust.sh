#!/bin/sh

set -euo pipefail

export CARGO_HOME="$(pwd)"/.cargo
export XARGO_HOME="$(pwd)"/.xargo
export RUSTUP_HOME="$(pwd)"/.rustup

cd repo/cargo-psp/
cargo build
cd ../..

PATH="$(realpath repo)/target/debug:$PATH"

cd repo/ci/tests
cargo psp
cd ../../..

cp -r repo/ci/tests/target/mipsel-sony-psp/debug/* rust-build-dir
