#!/bin/bash

set -euo pipefail

# If NO_CACHE is *not* set, then setup the cache directories
if [ -z "${NO_CACHE:-}" ]; then
    # Cache only for normal builds
    export CARGO_HOME="$(pwd)"/.cargo
    export RUSTUP_HOME="$(pwd)"/.rustup
fi

pwd
echo CARGO_HOME: $CARGO_HOME
echo RUSTUP_HOME: $RUSTUP_HOME

sleep 1000000

rustup set profile minimal
rustup update --no-self-update $RUSTUP_TOOLCHAIN

# Install rust-src if needed.
if ! rustup component list --installed | grep -q rust-src; then
    rustup component add rust-src
fi

# Test formatting
rustup component add rustfmt
pushd repo/
cargo fmt --check
status=$?
if test $status -ne 0
    then echo "Formatting errors: Please run cargo fmt on your changes"
    exit 1
fi
popd

# build cargo-psp
pushd repo/cargo-psp/
cargo build
popd

PATH="$(pwd)/repo/target/debug:$PATH"

# build the test project
pushd repo/ci/tests
cargo psp
popd

# Make the output directory, in case it is not specified as an output for this
# concourse task.
mkdir -p rust-build-dir

cp -r repo/ci/tests/target/mipsel-sony-psp/debug/* rust-build-dir
