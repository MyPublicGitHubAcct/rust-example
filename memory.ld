/* Linker script for the STM32F746g */

ENTRY(Reset_Handler)

MEMORY
{
    RAM (xrw)  : ORIGIN = 0x20000000, LENGTH = 320K
    FLASH (rx) : ORIGIN = 0x8000000, LENGTH  = 1024K
}

_start_of_stack = ORIGIN(RAM) + LENGTH(RAM); /* end of RAM */
_min_stack_size = 0x400; /* 1KB */
_min_heap_size  = 0x400; /* 1KB */

/*
1. Define where each section (.text, .data, .bss) should be placed in memory.

2. The linker assigns addresses based on the SECTIONS command, creating an ELF file with exact 
memory locations. For example, if FLASH starts at 0x08000000 and the .text is placed in FLASH 
then the linker will assign memory address starting from 0x08000000 for the .text section.

3. The ELF file guides the programming tool to correctly place the sections in the microcontroller
ensuring proper execution of the code.
*/
SECTIONS
{
    /* Functions, interrupt service routines and other compiled code are stored here */
    .text :
    {
        /* 
        ALIGN is used in this case to align the next address to a multiple of 4 bytes.
        So if the location counter is 0x0800 0021 for *(.text.*), then it will align the 
        memory address to be 0x0800 0024 (next multiple of 4)
        */
        . = ORIGIN(FLASH);
        LONG(_start_of_stack); /* Initial stack address pointer should come first in FLASH */
        KEEP(*(.isr_vector));  /* Tell the compiler to retain this section in the ELF file */
        *(.text)
        *(.text*)
        . = ALIGN(4);
    } > FLASH

    /* Read-only data */
    .rodata :
    {
        . = ALIGN(4);
        *(.rodata)
        *(.rodata*)
        . = ALIGN(4);
    } > FLASH

    /* 
    Initialized data will be in FLASH and it will also be in RAM during runtime.
    This data will be stored initially in FLASH before being copied to RAM during startup.

    First we specify execution memory address and then the load memory address.
    The syntax is `<execution memory address> AT> <load memory address>
    */
    .data :
    {
        _sidata = LOADADDR(.data); /* This return the LMA (FLASH) memory address */
        . = ALIGN(4);
        _sdata = .; /* Start of data section in VMA (virtual memory address which is the RAM) */
        *(.data)
        *(.data*)
        . = ALIGN(4);
        _edata = .; /* Start of data section in LMA (load memory address which is the FLASH) */
    } > RAM AT> FLASH

    /* Uninitialized data */
    .bss :
    {
        . = ALIGN(4);
        _sbss = .;
        *(.bss)
        *(.bss*)
        . = ALIGN(4);
        _ebss = .;
    } > RAM

    .ram_usage_check :
    {
        . = ALIGN(8);
        . = . + _min_stack_size;
        . = . + _min_heap_size;
        . = ALIGN(8);
    } > RAM
}

INCLUDE "./svd/device_STM32F746.x"
