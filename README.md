
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

Compiling a `--release` binary, we get ([playground, use the "..." button next to *Run* to select *Show assembly*](https://play.rust-lang.org/?version=stable&mode=release&edition=2018)):

```asm
duplicate:
	leal	(%rdi,%rdi), %eax
	retq
```

2 instructions, while the equivalent C++ code

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

is 2 also instructions, so Rust is on par when doing a release build (and not a debug build).

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
