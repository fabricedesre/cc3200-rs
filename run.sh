#!/bin/bash

OPT=debug
for opt in $@; do
    if [ x"$opt" == x"--release" ]; then
        OPT=release
    fi
done

TARGET=thumbv7em-none-eabi
ELF_DIR=target/${TARGET}/${OPT}
FIRMWARE_ELF=${ELF_DIR}/examples/blinky

set -x
arm-none-eabi-gdb -x gdbinit ${FIRMWARE_ELF}
