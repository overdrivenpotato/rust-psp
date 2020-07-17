#!/bin/bash
set -euo

if [ -d repo/ci/ ]; then
    export PREFIX="repo/"
    export HOMEDIR="$(pwd)"
else
    export PREFIX="rust-psp/"
    export HOMEDIR="${HOME}"
fi

export CARGO_HOME="${HOMEDIR}"/.cargo
export XARGO_HOME="${HOMEDIR}"/.xargo

pushd ${PREFIX}cargo-psp/
cargo build --release
popd

pushd ${PREFIX}ci/tests
${HOMEDIR}/${PREFIX}target/release/cargo-psp
popd

cp -r ${PREFIX}ci/tests/target/mipsel-sony-psp/release/* release/
