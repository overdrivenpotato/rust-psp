#!/bin/bash

set -euo

export CARGO_HOME="$(pwd)"/.cargo
export XARGO_HOME="$(pwd)"/.xargo

pushd repo/cargo-psp/
cargo build --release
popd

pushd repo/ci/tests
/repo/cargo-psp/target/release/cargo-psp
popd

cp -r repo/ci/tests/target/mipsel-sony-psp/release/* release/
