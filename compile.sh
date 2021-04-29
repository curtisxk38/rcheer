#!/bin/bash
cargo run $1
gcc -c output.s
gcc output.o
./a.out