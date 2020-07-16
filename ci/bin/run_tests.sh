#!/bin/bash

# Without this line, the tests will always pass.
set -ueo pipefail

[ -d ~/rust-psp/ci/ ] && cd ~/rust-psp/ci/

BUILD_DIR="target/mipsel-sony-psp/debug/"
LOG_FILE="${BUILD_DIR}/psp_output_file.log"

TEST_NAME="$1"

cd tests/"${TEST_NAME}"/

cargo psp
PPSSPPHeadless --timeout=5 ${BUILD_DIR}/EBOOT.PBP &> /dev/null

cat ${LOG_FILE}

SUCCESS=$(tail -n 1 ${LOG_FILE})
if [[ "${SUCCESS}" != "FINAL_SUCCESS" ]]; then
    echo "!!! Test '${TEST_NAME}' failed! !!!"
    exit 1
fi
