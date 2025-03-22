#![no_std]
#![no_main]

mod vector_table;

use core::panic::PanicInfo;
use cortex_m_rt::{entry, exception, ExceptionFrame};

use shared::firmware::addresses;
use shared::bootjump::jump_to_loader;
use drivers::stm32f4_rcc::RccHandle;

// The DefaultHandler definition belongs in main.rs, not vector_table.rs
#[exception]
unsafe fn DefaultHandler(_irqn: i16) {
    // Default handler for all unhandled interrupts
    loop {
        cortex_m::asm::nop();
    }
}

fn system_init() {
    // Initialize system clocks and peripherals
    if let Ok(rcc) = RccHandle::new() {
        // Configure system clock
        // ...
    }

    // Other initialization
}

#[entry]
fn main() -> ! {
    // Basic system initialization
    system_init();
    
    // Check if the loader is valid
    let loader_valid = unsafe { *(addresses::LOADER_ADDR as *const u32) != 0xFFFFFFFF };
    
    if loader_valid {
        // Jump to loader if valid
        jump_to_loader();
    } else {
        // Jump to updater if loader is invalid
        // ...
    }
    
    // We should never reach here
    loop {
        cortex_m::asm::nop();
    }
}

#[exception]
unsafe fn HardFault(_ef: &ExceptionFrame) -> ! {
    // Hard fault handler
    loop {
        cortex_m::asm::nop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Panic handler
    loop {
        cortex_m::asm::nop();
    }
}