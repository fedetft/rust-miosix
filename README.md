
Getting rust to work in Miosix
==============================

1) Install miosix toolchain
2) Install rustup
3) $ rustup target add thumbv7m-none-eabi
4) $ git submodule init && git submodule update
5) $ rustc -C panic=abort --target thumbv7m-none-eabi rust.rs
6) $ make


Is it good?
===========

How does Rust it translate to asm?

```rust
#[no_mangle]
pub fn duplicate(a: u32) -> u32 {
    return a * 2;
}
```

```asm
sub     sp, #8
adds    r1, r0, r0
mov     r2, r1
cmp     r1, r0
str     r2, [sp, #4]
bcc.n   0x64004160 <duplicate+20>
b.n     0x6400415a <duplicate+14>
ldr     r0, [sp, #4]
add     sp, #8
bx      lr
movw    r0, #62992      ; 0xf610
movt    r0, #25601      ; 0x6401
movw    r2, #62976      ; 0xf600
movt    r2, #25601      ; 0x6401
movs    r1, #33 ; 0x21
bl      0x64004778 <core::panicking::panic>
```

16 instructions, while the equivalent C++ code

```C++
unsigned int duplicate(unsigned int a)
{
    return a * 2;
}
```

```asm
lsls    r0, r0, #1
bx      lr
```

is 2 instructions, so Rust has still room for improvement.
Maybe rustc has a flag to enable compiler optimizations I don't know.


Is it possible to compile with the std library?
===============================================

Seems to not work, an example with a println doesn't even arrive at not linking with Miosix, it just doesn't compile.
Rustc says it can't find std, but rustup says std is installed.

```console
$ cat rust.rs
#![crate_type = "staticlib"]
#[no_mangle]
pub fn print() {
    println!("stdio is working");
}

$ rustc --target thumbv7m-none-eabi rust.rs
error[E0463]: can't find crate for `std`
  |
  = note: the `thumbv7m-none-eabi` target may not be installed

$ rustup target add thumbv7m-none-eabi
info: component 'rust-std' for target 'thumbv7m-none-eabi' is up to date
```
