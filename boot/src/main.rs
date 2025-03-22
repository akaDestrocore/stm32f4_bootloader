#![no_std]
#![no_main]

mod interrupts; // Включаем модуль с обработчиками прерываний

use core::panic::PanicInfo;
use cortex_m_rt::entry;

use shared::firmware::addresses;
use shared::bootjump;

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

// Точка входа в программу
#[entry]
fn main() -> ! {
    // Проверяем валидность загрузчика
    let loader_valid = unsafe { *(addresses::LOADER_ADDR as *const u32) != 0xFFFFFFFF };
    
    if loader_valid {
        // Переход в загрузчик приложений
        bootjump::jump_to_loader();
    } else {
        // Переход в программу обновления если загрузчик не валиден
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