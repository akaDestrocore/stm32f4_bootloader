#![no_std]
#![no_main]

mod interrupts;

use core::panic::PanicInfo;
use cortex_m_rt::entry;

use shared::firmware::addresses;
use shared::bootjump;
use drivers::stm32f4_rcc::RccHandle;

// Обработчик прерываний по умолчанию
#[cortex_m_rt::exception]
unsafe fn DefaultHandler(_irqn: i16) {
    loop {
        cortex_m::asm::nop();
    }
}

// Обработчик Hard Fault
#[cortex_m_rt::exception]
unsafe fn HardFault(_ef: &cortex_m_rt::ExceptionFrame) -> ! {
    loop {
        cortex_m::asm::nop();
    }
}

// Инициализация системы
fn system_init() {
    if let Ok(rcc) = RccHandle::new() {
        // Настройка системных тактов
        // ...
    }
}

// Точка входа в программу
#[entry]
fn main() -> ! {
    // Инициализация системы
    system_init();
    
    // Проверка валидности приложения
    let app_valid = unsafe { *(addresses::APP_ADDR as *const u32) != 0xFFFFFFFF };
    
    if app_valid {
        // Переход в приложение если оно валидно
        bootjump::jump_to_app();
    } else {
        // Переход в программу обновления если приложение не валидно
        bootjump::jump_to_updater();
    }
    
    // Этот код никогда не должен выполняться
    loop {
        cortex_m::asm::nop();
    }
}

// Обработчик паники
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        cortex_m::asm::nop();
    }
}