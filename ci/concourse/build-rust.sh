#!/bin/bash

export CARGO_HOME="$(pwd)"/.cargo
export XARGO_HOME="$(pwd)"/.xargo

cd repo/ci/tests/psp-ci-test
make
