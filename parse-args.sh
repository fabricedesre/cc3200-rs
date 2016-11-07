# Helper script used by build.sh, run.sh, and flash.sh for parsing arguments

EXAMPLE=blinky
EXAMPLE_ARG="--example ${EXAMPLE}"
ARGS="$@"
OPT=debug
while [[ $# -gt 0 ]]; do
    case "$1" in
        --release)
            OPT=release
            ;;
        --example)
            shift
            EXAMPLE="$1"
            EXAMPLE_ARG=
            ;;
    esac
    shift
done

TARGET=thumbv7em-none-eabi
ELF_DIR=target/${TARGET}/${OPT}
FIRMWARE_ELF=${ELF_DIR}/examples/${EXAMPLE}
FIRMWARE_BIN=${ELF_DIR}/examples/${EXAMPLE}.bin

