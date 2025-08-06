# Rust Example

This is intended to run on STM32F4 hardware.

## Installing Rust for STM32

Though most activities will be done via [cargo](https://doc.rust-lang.org/nightly/cargo/), the [rustc book](https://doc.rust-lang.org/nightly/rustc/) includes a section on [platform support](https://doc.rust-lang.org/nightly/rustc/platform-support.html).

To see the list of targets STM32 supports on Rust, use this command:

```rust
rustup target list
```

Included in the output, items already installed will include text like "(installed)", as shown below.

```zsh
aarch64-apple-darwin (installed)
```

The following relevant, but uninstalled targets will also be displayed.

```zsh
thumbv6m-none-eabi
thumbv7em-none-eabi
thumbv7em-none-eabihf
thumbv7m-none-eabi
```

- _thumbv7_ refers to to the ARM Cortex-M4 or Cortex-M7 (thumb) instruction set, the ARMv7E-M architecture. Similarly, _thumbv6_ refers to to the ARM Cortex-M0 or Cortex-M0+.

- _none_ describes a target that has no operating system (OS), commonly referred to as '_bare metal_'. 

    __Note__: One result of not using an OS is that only the [Rust core library](https://doc.rust-lang.org/stable/core/) is available. The [Rust Standard Library](https://doc.rust-lang.org/std/) is not.

- _eabi_ stands for embedded application binary interface.

- _hf_ means the target processor is equipped with __hardware__ floating-point support.

This project will be run on STM32F4 and STM32F7 hardware. To install ```thumbv7em-none-eabi```, run this command:

```zsh
rustup target add thumbv7em-none-eabi
```

To see the complete list of STM32 components on the host system, use this command:

```rust
rustup components list
```

This list will now include

```zsh
rust-std-thumbv7em-none-eabi (installed)
```


## Example Project

Create the project as you would any other Rust project.

```zsh
cargo new rust-example
cd rust-example
cargo build --target thumbv7em-none-eabi
```

There will, of course, be errors as the default Rust application project type includes the standard library (std), which includes [macros](https://doc.rust-lang.org/book/ch20-05-macros.html) like ```println!()```. 

Additionally, ```#[panic_handler]``` is missing, so this is considered unsafe Rust. See [this](https://doc.rust-lang.org/nomicon/panic-handler.html).

To be able to generate an ELF file, we need to do several things


### No Standard Library

To tell the compiler to not use the std, put the following attribute before ```
main()```.

```rust
#![no_std]
```

_Attributes_ in Rust provide instructions and metadata to the compiler. This includes controlling conditional compilation, deriving traits, managing test cases, etc. Details can be found [here](https://doc.rust-lang.org/reference/attributes.html).


### The Loop

An infinite _loop_ is needed to run program(s) on hardware with no OS. Rust has one built into the core language, which can only be stopped by _break_, _return_, or an _error/panic_. This could be done like:

Given the target we cannot use the Rust default ```main()``` as it relies upon std. Use of ```#![no_main]``` cures this and allows the definition of a custom entry point.

To avoid potential linking problems due to name mangling, the ```#[no_mangle]``` item level attribute should be used.


Now the project will compile & __main.rs__ will look like this.

```rust
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
fn main() {
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
```

__Note__: The never type, ```!``` represents a computation that will never resolve to a value or return control to a function's caller. This is often used for panic.


### Checking and Building

The code above will not compile using MacOS as the target, but will using Cortex-M4.

To check for errors, use the ```--target``` flag.

```zsh
cargo check --target thumbv7em-none-eabi
```

If there are no errors, you can build the project.

```zsh
cargo build --target thumbv7em-none-eabi
```

The ELF file, in this examples, can be found here - ```/Volumes/SomeDrive/SourceCode/rust-example/target/thumbv7em-none-eabi/debug/rust-example```.

To avoid the need to use the ```--target``` flag each time, make a top-level directory called __.cargo__ and add a file named _config.toml_. In that file, add the following:

```zsh
[build]
target = "thumbv7em-none-eabi"
```

Now, commands like the following can be used without stating the target.

```zsh
cargo check
cargo build
cargo clean
```

### The ELF file

An ELF file includes the following:

- ELF Header = metadata about the file
- Program header = information about the segments that need to be loaded
- Sections (programs):
  - .text   = executable code
  - .data   = initialized data
  - .bss    = uninitialized data (typically empty)
  - .rodata = read-only data
- Sections (debugging information):
  - .symtab = symbol table
  - .strtab = string table
- Section header table = describes sections within the file

Additional sections can be added with the linker (which is scripted).

Cargo includes tools that can be used to inspect an ELF file. To install these, use:

```zsh
cargo install cargo-binutils
rustup component add llvm-tools
```

Now, the following commands can be used...

```zsh
cargo objdump -- -h <elf file> # Overview of the sections in the ELF.
cargo readobj -- -S <elf file> # Information about each section of the ELF.
```
