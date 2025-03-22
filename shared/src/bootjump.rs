#![no_std]

use core::ptr;
use cortex_m::peripheral::SCB;
use cortex_m::interrupt;
use cortex_m::register::msp;

use crate::firmware::addresses;

/// Переход в загрузчик приложений
pub fn jump_to_loader() -> ! {
    // Получаем адрес обработчика сброса из таблицы векторов
    let reset_vector_addr = addresses::LOADER_ADDR + 4; // Vector table + 4 is reset handler
    let reset_vector = unsafe { 
        let func_ptr = ptr::read_volatile(reset_vector_addr as *const u32);
        core::mem::transmute::<u32, extern "C" fn() -> !>(func_ptr)
    };
    
    unsafe {
        // Отключаем прерывания
        interrupt::disable();
        
        // Устанавливаем таблицу векторов на новый адрес
        let scb = SCB::ptr();
        (*scb).vtor.write(addresses::LOADER_ADDR);
        
        // Сбрасываем указатель стека
        let stack_addr = ptr::read_volatile(addresses::LOADER_ADDR as *const u32);
        msp::write(stack_addr);
        
        // Переходим к обработчику сброса
        reset_vector();
    }
    
    // Этот код никогда не должен выполняться
    loop {
        cortex_m::asm::nop();
    }
}

/// Переход в программу обновления
pub fn jump_to_updater() -> ! {
    let reset_vector_addr = addresses::UPDATER_ADDR + 4;
    let reset_vector = unsafe { 
        let func_ptr = ptr::read_volatile(reset_vector_addr as *const u32);
        core::mem::transmute::<u32, extern "C" fn() -> !>(func_ptr)
    };
    
    unsafe {
        interrupt::disable();
        
        // Доступ к регистру VTOR через указатель SCB
        let scb = SCB::ptr();
        (*scb).vtor.write(addresses::UPDATER_ADDR);
        
        let stack_addr = ptr::read_volatile(addresses::UPDATER_ADDR as *const u32);
        msp::write(stack_addr);
        
        reset_vector();
    }
    
    loop {
        cortex_m::asm::nop();
    }
}

/// Переход в основное приложение
pub fn jump_to_app() -> ! {
    let reset_vector_addr = addresses::APP_ADDR + 4;
    let reset_vector = unsafe { 
        let func_ptr = ptr::read_volatile(reset_vector_addr as *const u32);
        core::mem::transmute::<u32, extern "C" fn() -> !>(func_ptr)
    };
    
    unsafe {
        interrupt::disable();
        
        // Доступ к регистру VTOR через указатель SCB
        let scb = SCB::ptr();
        (*scb).vtor.write(addresses::APP_ADDR);
        
        let stack_addr = ptr::read_volatile(addresses::APP_ADDR as *const u32);
        msp::write(stack_addr);
        
        reset_vector();
    }
    
    loop {
        cortex_m::asm::nop();
    }
}