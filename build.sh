#!/bin/bash

set -e

. ./parse-args.sh

rm -f ${FIRMWARE_ELF}
rm -f ${FIRMWARE_BIN}

set -x
xargo build --target=${TARGET} ${ARGS} ${EXAMPLE_ARG}
arm-none-eabi-size ${FIRMWARE_ELF}
arm-none-eabi-objcopy -O binary ${FIRMWARE_ELF} ${FIRMWARE_BIN}
