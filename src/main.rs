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
