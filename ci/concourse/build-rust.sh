#!/bin/bash

set -euo pipefail

# If NO_CACHE is *not* set, then setup the cache directories
if [ -z "${NO_CACHE:-}" ]; then
    # Cache only for normal builds

    # macOS Concourse runners make a new volume every time a task is run.
    # These volumes are named randomly every time, which changes where crate
    # source files are stored. This leads cargo to believe the files were
    # changed, so it triggers a full rebuild. Instead, we can use the default
    # directory for macOS runners. The rust-src component is also affected as
    # well, so we must exclude both CARGO_HOME and RUSTUP_HOME.
    #
    # On Linux, Concourse re-uses container paths so this issue is not relevant.
    # This is probably either a Concourse or Cargo bug.
    if ! [ $(uname) = Darwin ]; then
        export CARGO_HOME="$(pwd)"/.cargo
        export RUSTUP_HOME="$(pwd)"/.rustup
    fi
fi

rustup set profile minimal
rustup update --no-self-update $RUSTUP_TOOLCHAIN

# Install rust-src if needed.
if ! rustup component list --installed | grep -q rust-src; then
    rustup component add rust-src
fi

# Test formatting
rustup component add rustfmt
{
    cargo fmt --check --message-format=short --manifest-path=repo/cargo-psp/Cargo.toml;
    # TODO: remove `-ppsp` after formatting new workspace
    cargo fmt --check --message-format=short --manifest-path=repo/Cargo.toml -ppsp;
}
status=$?
if test $status -ne 0
    then echo "Formatting errors: Please run cargo fmt on your changes"
    exit 1
fi

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
