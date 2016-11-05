#!/bin/bash

. ./parse-args.sh

set -x
cc3200tool -p /dev/ttyUSB1 --sop2 ~dtr --reset prompt write_file ${FIRMWARE_BIN} /sys/mcuimg.bin
