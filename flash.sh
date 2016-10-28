#!/bin/sh

TARGET=thumbv7em-none-eabi
ELF_DIR=target/${TARGET}/debug
FIRMWARE_ELF=${ELF_DIR}/firmware
FIRMWARE_BIN=${ELF_DIR}/firmware.bin

set -x
cc3200tool -p /dev/ttyUSB1 --sop2 ~dtr --reset prompt write_file ${FIRMWARE_BIN} /sys/mcuimg.bin
