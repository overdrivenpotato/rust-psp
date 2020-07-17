#!/bin/bash
set -euxo pipefail

if [ -d repo/ci/ ]; then
    CI="1"
else
    CI="0"
fi

if true; then
    RELEASE="1"
else
    RELEASE="0"
fi

if [ "$CI" = "1" ]; then
    export PREFIX="repo/"
    export HOMEDIR="$(pwd)"
    export CARGO_HOME="${HOMEDIR}"/.cargo
    export XARGO_HOME="${HOMEDIR}"/.xargo
else
    export PREFIX="rust-psp/"
    export HOMEDIR="${HOME}"
fi


pushd ${PREFIX}cargo-psp/
if [ "$RELEASE" = "1" ]; then
    cargo build --release
else
    cargo build
fi
popd

PATH="${HOMEDIR}/${PREFIX}/target/release:${PATH}"

pushd ${PREFIX}ci/tests
[ -f Xargo.toml ] && rm Xargo.toml
if [ "$RELEASE" = "1" ]; then
    # TODO: add release flag? did not work with it added.
    ${HOMEDIR}/${PREFIX}target/release/cargo-psp
else
    ${HOMEDIR}/${PREFIX}target/debug/cargo-psp
fi
popd

if [ "$CI" = "1" ]; then
    cp -r ${PREFIX}ci/tests/target/mipsel-sony-psp/debug/* debug/
fi
