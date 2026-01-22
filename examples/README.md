# Examples: Rust Baremetal

The examples in this directory demonstrate how to do basic GPIO and serial
operations in baremetal no_std Rust.

Key Tasks:
- Document Rust toolchain install and setup
- Copy GPIO, UART, and USB CDC driver code from Bao1x bootloader
- Learn how to sign binaries to make the Bao1x bootloader happy
- Learn how to make the signed binaries into UF2 files


## Build Examples

Do this:

```
cargo build --example blinky
```


## Check the ELF Binary

Do this:

```
$ objdump -h target/riscv32imac-unknown-none-elf/debug/examples/blinky

target/riscv32imac-unknown-none-elf/debug/examples/blinky:     file format elf32-little

Sections:
Idx Name          Size      VMA       LMA       File off  Algn
  0 .init         00000056  60060400  60060400  00000400  2**1
                  CONTENTS, ALLOC, LOAD, READONLY, CODE
  1 .text         00000000  60060458  60060458  00000458  2**2
                  CONTENTS, ALLOC, LOAD, READONLY, CODE
  2 .text.main    00000002  60060458  60060458  00000458  2**1
                  CONTENTS, ALLOC, LOAD, READONLY, CODE
  3 .text.memset  0000006c  6006045a  6006045a  0000045a  2**1
                  CONTENTS, ALLOC, LOAD, READONLY, CODE
  4 .text.memcpy  00000008  600604c6  600604c6  000004c6  2**1
                  CONTENTS, ALLOC, LOAD, READONLY, CODE
  5 .text._ZN17compiler_builtins3mem6memcpy17h818a223eda01212cE 000001a2  600604ce  600604ce  000004ce  2**1
                  CONTENTS, ALLOC, LOAD, READONLY, CODE
  6 .eh_frame     00000064  60060670  60060670  00000670  2**2
                  CONTENTS, ALLOC, LOAD, READONLY, DATA
  7 .data         00000000  61000000  61000000  000006d4  2**2
                  CONTENTS, ALLOC, LOAD, READONLY, DATA
  8 .bss          00000000  61000000  61000000  000006d4  2**2
                  ALLOC
  9 .heap         00080000  61000000  61000000  00001000  2**2
                  ALLOC
 10 .stack        00008000  61080000  61080000  00001000  2**2
                  ALLOC
 11 .comment      0000008b  00000000  00000000  00001000  2**0
                  CONTENTS, READONLY
 12 .riscv.attributes 0000004e  00000000  00000000  0000108b  2**0
                  CONTENTS, READONLY
```
