# RCheer
A compiler

Compile by running `cargo run filename.ch`

This will generate a x86 assembly .s file, that you can assemble/link with gcc

```
gcc -c output.s
gcc output.o
./a.out
```

The `compile.sh` script will do all of the above, so you can run
`./compile.sh filename.ch` to compile a program and execute it

# Tests
The integration tests pass a Cheer program to the compiler which outputs assembly. Then GCC is used to compile the assembly into an executable, and then the executable is run.

By default, cargo runs tests in parallel. This causes the integration tests to mess up

`RUST_TEST_THREADS=1 cargo test`

## Notes
C to asm:
```
gcc -S test.c
```

asm to executable:
```
gcc -c test.s
gcc test.o
```

see asm of executable:
```
objdump -d a.out
```

gdb set breakpoint in asm
```
break *main
run a.out
stepi
```

gdb inspect stuff
```
info frame
i r $sp
i r $rax
x/5x $sp
```

syscall stuff
```
https://stackoverflow.com/questions/12806584/what-is-better-int-0x80-or-syscall-in-32-bit-code-on-linux
```