#![no_std]

use core::ptr;
use cortex_m::asm;
use cortex_m::peripheral::SCB;

use crate::firmware::addresses;

/// Jump to the application loader
pub fn jump_to_loader() -> ! {
    // Get the reset handler address from the vector table
    let reset_vector_addr = addresses::LOADER_ADDR + 4; // Vector table + 4 is reset handler
    let reset_vector = unsafe { 
        let func_ptr = ptr::read_volatile(reset_vector_addr as *const u32);
        core::mem::transmute::<u32, unsafe extern "C" fn() -> !>(func_ptr)
    };
    
    unsafe {
        // Disable interrupts
        asm::disable_irq();
        
        // Get SCB peripheral
        let scb = &mut *SCB::ptr();
        
        // Reset peripherals if needed
        // ...
        
        // Set vector table to the new address
        scb.vtor.write(addresses::LOADER_ADDR);
        
        // Reset stack pointer
        let stack_addr = ptr::read_volatile(addresses::LOADER_ADDR as *const u32);
        asm::msr("msp", stack_addr);
        
        // Jump to reset handler
        reset_vector();
    }
    
    // We should never reach here
    loop {
        asm::nop();
    }
}

/// Jump to the updater
pub fn jump_to_updater() -> ! {
    let reset_vector_addr = addresses::UPDATER_ADDR + 4;
    let reset_vector = unsafe { 
        let func_ptr = ptr::read_volatile(reset_vector_addr as *const u32);
        core::mem::transmute::<u32, unsafe extern "C" fn() -> !>(func_ptr)
    };
    
    unsafe {
        asm::disable_irq();
        
        let scb = &mut *SCB::ptr();
        scb.vtor.write(addresses::UPDATER_ADDR);
        
        let stack_addr = ptr::read_volatile(addresses::UPDATER_ADDR as *const u32);
        asm::msr("msp", stack_addr);
        
        reset_vector();
    }
    
    loop {
        asm::nop();
    }
}

/// Jump to the application
pub fn jump_to_app() -> ! {
    let reset_vector_addr = addresses::APP_ADDR + 4;
    let reset_vector = unsafe { 
        let func_ptr = ptr::read_volatile(reset_vector_addr as *const u32);
        core::mem::transmute::<u32, unsafe extern "C" fn() -> !>(func_ptr)
    };
    
    unsafe {
        asm::disable_irq();
        
        let scb = &mut *SCB::ptr();
        scb.vtor.write(addresses::APP_ADDR);
        
        let stack_addr = ptr::read_volatile(addresses::APP_ADDR as *const u32);
        asm::msr("msp", stack_addr);
        
        reset_vector();
    }
    
    loop {
        asm::nop();
    }
}