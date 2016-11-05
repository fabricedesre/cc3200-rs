#!/bin/bash

. ./parse-args.sh

set -x
arm-none-eabi-gdb -x gdbinit ${FIRMWARE_ELF}
