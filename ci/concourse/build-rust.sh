#!/bin/bash
set -euxo pipefail

. "$(dirname $0)"/env.sh

pushd ${PREFIX}/cargo-psp/
if [ "$OPT_LEVEL" = "release" ]; then
    cargo build --release
else
    cargo build
fi
popd

PATH="${HOMEDIR}/${PREFIX}/target/${OPT_LEVEL}:${PATH}"

pushd ${PREFIX}/ci/tests

[ -f Xargo.toml ] && rm Xargo.toml

if [ "$OPT_LEVEL" = "release" ]; then
    cargo psp --release
else
    cargo psp
fi
popd

if [ "$CI" = "1" ]; then
    cp -r ${PREFIX}/ci/tests/target/mipsel-sony-psp/${OPT_LEVEL}/* release/
fi
