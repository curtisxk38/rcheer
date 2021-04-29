# RCheer
=====
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