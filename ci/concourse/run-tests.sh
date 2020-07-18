#!/bin/bash

# Fail on errors
set -euxo pipefail

. "$(dirname $0)"/env.sh

"$PPSSPP" "${BUILD_DIR}/EBOOT.PBP" --timeout=10 -r "${BUILD_DIR}/"

cat "${BUILD_DIR}"/psp_output_file.log

if [ "$(tail -n 1 "${BUILD_DIR}/psp_output_file.log")" == "FINAL_SUCCESS" ]; then \
    echo "Test passed";
else \
    echo "Test failed";
    exit -1
fi
