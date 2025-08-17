#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;

mod startup_stm32f746g; // tell the compiler to include this file

static mut SCORES_GLOBAL: [i32; 5] = [1, 2, 3, 4, 5];

const _NUMBERS: [i32; 5] = [1, 2, 3, 4, 5];

static mut BUFFER: [u8; 1024] = [0; 1024];

#[unsafe(no_mangle)]
fn main() {
    let mut _total_score = 0;

    unsafe {
        for score in SCORES_GLOBAL {
            _total_score += score;
        }
    }

    unsafe {
        BUFFER[0] = 100;
    }
    
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
