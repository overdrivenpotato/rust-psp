#!/bin/bash

set -euo pipefail

# If NO_CACHE is *not* set, then setup the cache directories
if [ -z "${NO_CACHE:-}" ]; then
    # Cache only for normal builds
    export CARGO_HOME="$(pwd)"/.cargo
    export XARGO_HOME="$(pwd)"/.xargo
    export RUSTUP_HOME="$(pwd)"/.rustup
fi

rustup set profile minimal
rustup update $RUSTUP_TOOLCHAIN

# Install rust-src if needed.
if ! rustup component list --installed | grep -q rust-src; then
    rustup component add rust-src
fi

pushd repo/cargo-psp/
cargo build
popd

PATH="$(realpath repo)/target/debug:$PATH"

pushd repo/ci/tests
cargo psp
popd

cp -r repo/ci/tests/target/mipsel-sony-psp/debug/* rust-build-dir
