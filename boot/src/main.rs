#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m::asm;
use cortex_m_rt::entry;

// Константы для адресов
const LOADER_ADDR: u32 = 0x08004000;

#[entry]
fn main() -> ! {
    // Самый простой код
    loop {
        asm::nop();
    }
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        asm::nop();
    }
}