#!/bin/bash

# Fail on errors
set -euo pipefail

/ppsspp/build-sdl/PPSSPPHeadless build/EBOOT.PHP --timeout=10 -r build/

cat build/psp_output_file.log

if [`tail -n 1 build/psp_output_file.log` == "FINAL_SUCCESS"]; then \
    echo "Test passed";
else \
    echo "Test failed";
    exit -1
fi
