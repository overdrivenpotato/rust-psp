if [ -d repo/ci/ ]; then
    export CI="1"
    export OPT_LEVEL="release"
    export BUILD_DIR="build"
    export PREFIX="repo"
    export PPSSPP="/ppsspp/build-sdl/PPSSPPHeadless"
    export HOMEDIR="$(pwd)"
    export CARGO_HOME="${HOMEDIR}"/.cargo
    export XARGO_HOME="${HOMEDIR}"/.xargo
else
    export CI="0"
    export OPT_LEVEL="debug"
    export PREFIX="rust-psp"
    export BUILD_DIR="ci/tests/target/mipsel-sony-psp/${OPT_LEVEL}"
    export PPSSPP="PPSSPPHeadless"
    export HOMEDIR="${HOME}"
fi
