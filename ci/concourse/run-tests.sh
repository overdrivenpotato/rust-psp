#!/bin/bash

set -euo pipefail

/ppsspp/build-sdl/PPSSPPHeadless rust-build-dir/test_cases.EBOOT.PBP --timeout=10 -r .

cat psp_output_file.log

if [ "$(tail -n 1 psp_output_file.log)" == FINAL_SUCCESS ]; then
    echo Test passed
else
    echo Test failed
    exit 1
fi
