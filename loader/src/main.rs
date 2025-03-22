#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr;
use cortex_m::asm;
use cortex_m_rt::entry;

// Константы для адресов
const APP_ADDR: u32 = 0x08020000;
const UPDATER_ADDR: u32 = 0x08008000;

#[entry]
fn main() -> ! {
    // Проверка валидности приложения
    let app_valid = unsafe { *(APP_ADDR as *const u32) != 0xFFFFFFFF };
    
    if app_valid {
        jump_to_address(APP_ADDR);
    } else {
        jump_to_address(UPDATER_ADDR);
    }
    
    // Этот код никогда не должен выполниться
    loop {
        asm::nop();
    }
}

// Простая функция для перехода по адресу
fn jump_to_address(addr: u32) -> ! {
    unsafe {
        // Отключаем прерывания
        cortex_m::interrupt::disable();
        
        // Устанавливаем таблицу векторов
        let scb = cortex_m::peripheral::SCB::ptr();
        (*scb).vtor.write(addr);
        
        // Устанавливаем указатель стека
        let stack_ptr = *(addr as *const u32);
        cortex_m::register::msp::write(stack_ptr);
        
        // Получаем адрес функции сброса
        let reset_addr = addr + 4;
        let reset_ptr = *(reset_addr as *const u32);
        
        // Переходим к функции
        let jump_fn: fn() -> ! = core::mem::transmute(reset_ptr);
        jump_fn();
    }
    
    // Код должен быть недостижим
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