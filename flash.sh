#!/bin/sh

OPT=debug
for opt in $@; do
    if [ x"$opt" == x"--release" ]; then
        OPT=release
    fi
done

TARGET=thumbv7em-none-eabi
ELF_DIR=target/${TARGET}/${OPT}
FIRMWARE_ELF=${ELF_DIR}/firmware
FIRMWARE_BIN=${ELF_DIR}/firmware.bin

set -x
cc3200tool -p /dev/ttyUSB1 --sop2 ~dtr --reset prompt write_file ${FIRMWARE_BIN} /sys/mcuimg.bin
