#!/bin/sh

TARGET=thumbv7em-none-eabi
ELF_DIR=target/${TARGET}/debug
FIRMWARE_ELF=${ELF_DIR}/firmware
FIRMWARE_BIN=${ELF_DIR}/firmware.bin

rm -f ${FIRMWARE_ELF}
rm -f ${FIRMWARE_BIN}

xargo build --target=${TARGET} "$@"

arm-none-eabi-size ${FIRMWARE_ELF}
arm-none-eabi-objcopy -O binary ${FIRMWARE_ELF} ${FIRMWARE_BIN}
