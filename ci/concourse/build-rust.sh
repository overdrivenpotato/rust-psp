#!/bin/bash

set -euo pipefail

export CARGO_HOME="$(pwd)"/.cargo
export XARGO_HOME="$(pwd)"/.xargo

pushd repo/cargo-psp/
cargo build
popd

PATH="$(realpath repo)/target/debug:$PATH"

pushd repo/ci/tests

cargo psp

popd

cp -r repo/ci/tests/target/mipsel-sony-psp/debug/* rust-build-dir
