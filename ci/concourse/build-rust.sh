#!/bin/bash

export CARGO_HOME="$(pwd)"/.cargo
export XARGO_HOME="$(pwd)"/.xargo

pushd repo/ci/tests/psp-ci-test
make
popd

cp -r repo/ci/tests/psp-ci-test/target/psp/release/* release/
