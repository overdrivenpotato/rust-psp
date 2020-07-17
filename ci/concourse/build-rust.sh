#!/bin/bash
set -euxo pipefail

if [ -d repo/ci/ ]; then
    CI="1"
else
    CI="0"
fi

if true; then
    RELEASE="release"
else
    RELEASE="debug"
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
if [ "$RELEASE" = "release" ]; then
    cargo build --release
else
    cargo build
fi
popd

PATH="${HOMEDIR}/${PREFIX}/target/${RELEASE}:${PATH}"

pushd ${PREFIX}ci/tests
[ -f Xargo.toml ] && rm Xargo.toml
# TODO: add release flag? did not work with it added.
if [ "$RELEASE" = "release" ]; then
    cargo-psp --release
else
    cargo-psp
fi
popd

if [ "$CI" = "1" ]; then
    cp -r ${PREFIX}ci/tests/target/mipsel-sony-psp/${RELEASE}/* release/
fi
