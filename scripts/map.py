#!/usr/bin/env python3

import argparse
import os
import re
import sys

def analyze(map_filename):
    pattern = re.compile('^\s+([^0 ]*)?\s*0x([0-9A-Fa-f]*)\s*0x([0-9A-Fa-f]*)\s*(\S*)')
    size = {}
    with open('firmware.map', 'r') as f:
        for line in f:
            if line.startswith('/DISCARD/'):
                break

            result = pattern.match(line)
            if not result:
                continue

            section = result.group(1)
            address = int(result.group(2), 16)
            if address == 0:
                continue
            length = int(result.group(3), 16)
            filename = os.path.basename(result.group(4))

            if length == 0:
                continue

            if not filename:
                filename = section
            if filename in size:
                size[filename] = size[filename] + length
            else:
                size[filename] = length
    return size

def print_sizes(size, hex):
    total = 0
    for file in sorted(size.items(), key=lambda x: x[1], reverse=True):
        sz = file[1]
        total += sz
        if hex:
            print('{:5x} {}'.format(sz, file[0]))
        else:
            print('{:5} {}'.format(sz, file[0]))
    if hex:
        print('Total = {:x}'.format(total))
    else:
        print('Total = {}'.format(total))

def main():
    parser = argparse.ArgumentParser(
        prog="map.py",
        usage="%(prog)s [options] map-filename",
        description="Analyze map file"
    )
    parser.add_argument(
        '-x', '--hex',
        dest='hex',
        action='store_true',
        help='Print sizes in hex'
    )
    parser.add_argument(
        'map_filename',
        type=str,
        help='Name of map file to analyze'
    )
    args = parser.parse_args(sys.argv[1:])

    print_sizes(analyze(args.map_filename), hex=args.hex)

main()
