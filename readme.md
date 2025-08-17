# Rust Example

This is intended to run on STM32F4 hardware.

## Resources

- [Rust book](https://doc.rust-lang.org/stable/book/) shows the basics of the Rust language, from the developer.
- [Cargo book](https://doc.rust-lang.org/cargo/print.html) is helpful to understand working with Rust.
- [Rust embedded book](https://docs.rust-embedded.org/book/intro/index.html)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) is set of examples.
- [Rustlings](https://rustlings.rust-lang.org) is a library used for learning the Rust language.
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/intro.html) is a list of design patterns intended to provide standard ways of accomplishing common tasks.
- [Embassy framework](https://github.com/embassy-rs/embassy) for embedded, but not used here.

## Installing Rust for STM32

Though most activities will be done via [cargo](https://doc.rust-lang.org/nightly/cargo/), the [rustc book](https://doc.rust-lang.org/nightly/rustc/) includes a section on [platform support](https://doc.rust-lang.org/nightly/rustc/platform-support.html).

### Download & run the script that installs the rustup tool

This will install cargo, clippy, rust-docs, rust-std, rustc, and rustfmt.

```zsh
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### Check the compiler version

```zsh
rustc --version
```

### Update Rust

```zsh
rustup update
```

### Uninstall Rust

```zsh
rustup self uninstall
```

### Install Cargo tools that can be used to inspect an ELF file

```zsh
cargo install cargo-binutils
rustup component add llvm-tools
```

Now, the following commands can be used...

```zsh
cargo objdump -- -h <elf file> # Overview of the sections in the ELF.
cargo readobj -- -all <elf file>       # Print all details of the ELF.
cargo readobj -- -x .data <elf file>   # Print contents of .data.
cargo readobj -- -x .rodata <elf file> # Print contents of .rodata.
cargo readobj -- -x .text <elf file>   # Print contents of .text.
cargo readobj -- -s <elf file>         # Print the symbol table.
```

### Show the list of targets STM32 supports on Rust

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

The code above will not compile to an ELF using MacOS as the target, but will using a different target. Cortex-M4 and Cortex-M7 are both in the Armv7E-M architecture family. _thumbv7em-none-eabi_, and _thumbv7em-none-eabihf_ are appropriate targets for this family. The latter is the same as the former except that it includes _hardware_ floating point support.

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
cargo fmt
cargo clippy
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

## Startup

Startup code runs before _main()_ and often includes:

- __Vector table__ - defines the initial stack pointer and the addresses of the interrupt and exception handlers.
- __Reset handler__ - the entry point for the program. This initializes the hardware and sets up the runtime environment.
- __Exception handlers__ - handle known exceptions.

Note that some parts of the startup code will be target dependent as it will need to, for example, place the vector table in the correct memory location. It will also initialize _the stack_ and _.data_ and _.bss_.

### The Linker

The linker used in this project [LLD](lld.llvm.org) is the default LLVM linker Rust uses for the _thumbv7em-none-eabi_, and _thumbv7em-none-eabihf_ targets.

It is possible to use an external linker such as the one provided in the GNU Arm Embedded Toolchain. To add that to a project, update ```.cargo/config.toml``` to include:

```toml
[target.<your-target>]
linker = "arm-none-eabi-ld"
```

### Linker flags

To enable linking, the following must be added to the ```.cargo/config.toml``` file, where _{linker-script.ld}_ is replaced by the name of your linker script.

```toml
[target.thumbv7em-none-eabi]
rustflags = [
  "-C", "link-arg=-T{linker-script.ld}"
]
```

In this project, the linker scipt is called _memory.ld_. Now the ```.cargo/config.toml``` file looks like the snippet below. __Note__ that this must be at the project level, outside of the _src_ folder.

```toml
[build]
target = "thumbv7em-none-eabi"

[target.thumbv7em-none-eabi]
rustflags = [
  "-C", "link-arg=-Tmemory.ld"
]
```

#### Optional external linker

To use use an external linker such as the one provided in the GNU Arm Embedded Toolchain, a line identifying the linker must be added, as below.

```toml
[target.thumbv7em-none-eabi]
linker = "arm-none-eabi-ld"
rustflags = [
  "-C", "link-arg=-T{linker-script.ld}"
]
```

### Linker script

This is a text file used by the linker to define and control the layout i.e., sections of the executable file, & where these should be placed in the MCU's memory. This will also set attributes such as mutability for each section.

In [GNU's linker documentation](https://ftp.gnu.org/old-gnu/Manuals/ld-2.9.1/html_node/ld_toc.html), the [Command Lanuage section](https://ftp.gnu.org/old-gnu/Manuals/ld-2.9.1/html_chapter/ld_3.html) includes information about linker scripts, including command that can be used.

Also, [LLVM's LLD documentation](https://lld.llvm.org/ELF/linker_script.html) shows their expectation for linker scripts.

#### MEMORY

The __MEMORY__ command defines the memeory regions on the target device.

The names "FLASH" and "RAM" are just names chosen, they are not part of any standard or specific to an MCU.

```rust
MEMORY
{
    FLASH (rx)   : ORIGIN = 0x8000000, LENGTH = 1024K
    RAM   (rwx)  : ORIGIN = 0x20000000, LENGTH = 320K
}
```

Some less common memory types are:
- _CCMRAM_ or _CCRAM_ is core-coupled memory (RAM), which is separate from regular SRAM and is helpful to achieve low-latency.
- _EEPROM_ is reprogrammable read-only memory.
- _BATTRAM_ is battery-backed RAM, which retains data when the device is turned off.

#### Types of Data

- __Read-only__ - (_.rodata_) is stored in ROM or Flash memmory & includes string literals, constant variables, static immutable variables.
- __Initialized__ - (_.data_) is stored in ROM, but is also copied to RAM at startup so the program can modify it at runtime. It includes initialized static _mutable_ (global) variables & stack (local initialized) variables. 
- __Unitialized__ - (_.bss_) is metadata describing size needed stored in ROM & includes unitialized global, static, and static mutable variables. During runtime, the startup code zeros out the .bass section in RAM.
- __Stack__ - holds local variables & function call items such as return addresses and paramters.
- __Heap__ - holds dynamically allocated memory such as Box and Vec.

#### Location Counter

The _location counter_ (.) represents the current _memory_ address within the section being processed. As the linker processes the SECTIONS, it automatically updates the _location counter_ to reflect teh current position in memory. You can use the _location counter_ to define the start or end of sections or to create gaps between sections by manipulating its value.

The _RAM address_, a.k.a. __Virtual Memory Address__ (VMA), is where the section is loded into RAM at runtime.

The _FLASH address_, a.k.a. __Load Memory Address__ (LMA), is where the section is stored in FLASH before being copied to RAM.

- _LOADADDR().data_ returns the FLASH address.

- _ALIGN(n)_ aligns the lcoation counter to the next multiple of __n__ bytes. If the current address is already aligned to __n__, it remains unchanged. _ALIGN(n)_ ensures that your code and data sections are correctly placed at their natural and appropriate memory boundaries, preventing misalignment and potential faults.

The ARM Cortex Mx stack pointer is "Full Descending"; it starts at the end of the memory and decrements. It is aligned to an eight byte boundary.

To see the memory, use ```cargo objdump```.

```zsh
cargo objdump -- -h target/thumbv7em-none-eabi/debug/rust-example

# Results in something like:

rust-example:   file format elf32-littlearm

Sections:
Idx Name             Size     VMA      LMA      Type
  0                  00000000 00000000 00000000 
  1 .text            00000484 08000000 08000000 TEXT
  2 .rodata          0000042c 08000484 08000484 DATA
  3 .ARM.exidx       00000010 080008b0 080008b0 DATA
  4 .data            00000014 20000000 080008c0 DATA
  5 .bss             00000400 20000014 20000014 BSS
  6 .ram_usage_check 00000804 20000414 20000414 DATA
  7 .debug_abbrev    00001e7e 00000000 00000000 DEBUG
  8 .debug_info      0002713a 00000000 00000000 DEBUG
  9 .debug_aranges   00001540 00000000 00000000 DEBUG
 10 .debug_ranges    00013518 00000000 00000000 DEBUG
 11 .debug_str       00041448 00000000 00000000 DEBUG
 12 .comment         00000099 00000000 00000000 
 13 .ARM.attributes  00000032 00000000 00000000 
 14 .debug_frame     00004c90 00000000 00000000 DEBUG
 15 .debug_line      000258aa 00000000 00000000 DEBUG
 16 .debug_loc       00000028 00000000 00000000 DEBUG
 17 .symtab          00000600 00000000 00000000 
 18 .shstrtab        000000cf 00000000 00000000 
 19 .strtab          00000fd2 00000000 00000000 
```

To see more information about the memory, use ```cargo readobj```.

```zsh
cargo readobj -- -S target/thumbv7em-none-eabi/debug/rust-example

# Results in something like:

File: rust-example
There are 20 section headers, starting at offset 0xcb744:

Section Headers:
  [Nr] Name              Type            Address  Off    Size   ES Flg Lk Inf Al
  [ 0]                   NULL            00000000 000000 000000 00      0   0  0
  [ 1] .text             PROGBITS        08000000 010000 000484 00  AX  0   0  4
  [ 2] .rodata           PROGBITS        08000484 010484 00042c 00 AMS  0   0  4
  [ 3] .ARM.exidx        ARM_EXIDX       080008b0 0108b0 000010 00  AL  1   0  4
  [ 4] .data             PROGBITS        20000000 020000 000014 00  WA  0   0  4
  [ 5] .bss              NOBITS          20000014 020014 000400 00  WA  0   0  1
  [ 6] .ram_usage_check  PROGBITS        20000414 020414 000804 00  WA  0   0  1
  [ 7] .debug_abbrev     PROGBITS        00000000 020c18 001e7e 00      0   0  1
  [ 8] .debug_info       PROGBITS        00000000 022a96 02713a 00      0   0  1
  [ 9] .debug_aranges    PROGBITS        00000000 049bd0 001540 00      0   0  1
  [10] .debug_ranges     PROGBITS        00000000 04b110 013518 00      0   0  1
  [11] .debug_str        PROGBITS        00000000 05e628 041448 01  MS  0   0  1
  [12] .comment          PROGBITS        00000000 09fa70 000099 01  MS  0   0  1
  [13] .ARM.attributes   ARM_ATTRIBUTES  00000000 09fb09 000032 00      0   0  1
  [14] .debug_frame      PROGBITS        00000000 09fb3c 004c90 00      0   0  4
  [15] .debug_line       PROGBITS        00000000 0a47cc 0258aa 00      0   0  1
  [16] .debug_loc        PROGBITS        00000000 0ca076 000028 00      0   0  1
  [17] .symtab           SYMTAB          00000000 0ca0a0 000600 10     19  82  4
  [18] .shstrtab         STRTAB          00000000 0ca6a0 0000cf 00      0   0  1
  [19] .strtab           STRTAB          00000000 0ca76f 000fd2 00      0   0  1
Key to Flags:
  W (write), A (alloc), X (execute), M (merge), S (strings), I (info),
  L (link order), O (extra OS processing required), G (group), T (TLS),
  C (compressed), x (unknown), o (OS specific), E (exclude),
  R (retain), y (purecode), p (processor specific)
```

#### The Vector Table

The vector table is a collection of addresses that point to _Interrupt Service Routines_ (ISR's, or interrupt handlers) and exception handlers. The processor looks for this table at a specific (well-defined) address in memory, usually at the beginning of the code space (commonly 0x00000000).

The size of the vector table is the number of interrupt requests (IRQ's) plus the number of exception handlers. Each will need its own memory address. The microcontroller's _reference_ manual (section 10.1.2 Interrupt and exception vectors, in this case) will show how many of each type.

The [svd-vector-gen](https://github.com/niekiran/svd-vector-gen/tree/main) tool can be used to create the vector table rather than manually doing this (which looks like no fun). To install this, use 

```zsh
cargo install svd-vector-gen
```

The svd file is provided by ST & retreiveable from [STM32CubeCLT](https://www.st.com/en/development-tools/stm32cubeclt.html). This is installed in ```/opt/ST/``` on mac.

The SVD files can be found in /opt/ST/STM32CubeCLT_1.19.0/STMicroelectronics_CMSIS_SVD. Make a folder and copy the file(s) needed into it. Now, run ```svd-vector-gen``` in that folder to create text files describing the vector table and the device.

```extern "C"``` must be used to conform to the ARM EABI's implementation of the C ABI. Effectively, you are telling the Rust compiler to generate functions that conform to conventions for parameter passing, stack frame setup and teardown (prologue and epilogue), and return value handling.

Without defining all of the handlers, the compiler will send errors and stop. This is fixed by either defining each handler or using the PROVIDE function to assign them to a default handler (until or if they are defined later).

The _PROVIDE_ directive in a linker script can be used to define a symbol as an alias for another symbol to fall back on if not found. Typically, this looks like:

```zsh
PROVIDE(TIM1_isr = default_handler);
```

In this case, if ```TIM1_isr``` is not defined elsewhere, it will default to ```default_handler```.

Since the list of PROVIDE commands was generated by XXX, we can add ```INCLUDE "./svd/device_STM32F746.x"``` to the bottom of the linker script, _memory.ld_ and compile without error.
