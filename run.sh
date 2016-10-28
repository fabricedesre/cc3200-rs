#!/bin/sh

TARGET=thumbv7em-none-eabi
ELF_DIR=target/${TARGET}/debug
FIRMWARE_ELF=${ELF_DIR}/firmware

set -x
arm-none-eabi-gdb -x gdbinit ${FIRMWARE_ELF}
