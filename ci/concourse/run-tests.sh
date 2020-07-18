#!/bin/bash

# Fail on errors
set -euxo pipefail

if true; then
    RELEASE="release"
else
    RELEASE="debug"
fi

if [ -d repo/ci/ ]; then
    CI="1"
    BUILD_DIR="build"
    PPSSPP="/ppsspp/build-sdl/PPSSPPHeadless"
else
    CI="0"
    BUILD_DIR="ci/tests/target/mipsel-sony-psp/${RELEASE}"
    PPSSPP="PPSSPPHeadless"
fi

"$PPSSPP" "${BUILD_DIR}/EBOOT.PBP" --timeout=10 -r "${BUILD_DIR}/"

cat "${BUILD_DIR}"/psp_output_file.log

if [ "$(tail -n 1 "${BUILD_DIR}/psp_output_file.log")" == "FINAL_SUCCESS" ]; then \
    echo "Test passed";
else \
    echo "Test failed";
    exit -1
fi
