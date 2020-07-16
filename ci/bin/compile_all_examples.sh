#!/bin/bash

# Without this line, the tests will always pass.
set -ueo pipefail

[ -d ~/rust-psp/examples/ ] && cd ~/rust-psp/examples/ || ( echo "No examples directory found" && exit 1 )

for entry in *; do
    if [ -d "$entry" ]; then
        pushd $entry 1>/dev/null
        echo "-- Compiling $entry --"
        cargo psp
        popd 1>/dev/null
    fi
done

echo "-- Finished successfully! --"
