#!/usr/bin/env bash
set -euxo pipefail

if [[ $# != 1 ]]; then
    echo "Incorrect argument count"
    exit 1
fi

if [[ $1 == "clean" ]]; then
    rm *.o out
else
    nasm $1 -felf64 -g -F dwarf -o out.o && ld out.o -o out
fi
