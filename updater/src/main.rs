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

// Основные состояния программы обновления
enum UpdaterState {
    Idle,
    ReceivingFirmware,
    VerifyingFirmware,
    InstallingFirmware,
}

// Точка входа в программу
#[entry]
fn main() -> ! {
    // Инициализация системы
    system_init();
    
    // Основной цикл программы обновления
    let mut state = UpdaterState::Idle;
    
    loop {
        match state {
            UpdaterState::Idle => {
                // Ожидание команды от пользователя
                // ...
            }
            UpdaterState::ReceivingFirmware => {
                // Получение прошивки
                // ...
            }
            UpdaterState::VerifyingFirmware => {
                // Проверка прошивки
                // ...
            }
            UpdaterState::InstallingFirmware => {
                // Установка прошивки
                // ...
                
                // После успешной установки перейти в загрузчик
                bootjump::jump_to_loader();
            }
        }
        
        // Выполнение дополнительных операций в цикле
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