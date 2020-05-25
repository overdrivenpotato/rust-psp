#!/bin/bash

# Fail on errors
set -e

/ppsspp/build-sdl/PPSSPPHeadless build/psp-ci-test.prx --timeout=10 -r build/
cat build/psp-ci-test.test
diff build/psp-ci-test.test repo/ci/tests/psp-ci-test/psp-ci-test.expected | tee build/psp-ci-test.result

if [`cat build/psp-ci-test.result` == ""]; then \
    echo "Test passed";
else \
    echo "Test failed";
    exit -1
fi
