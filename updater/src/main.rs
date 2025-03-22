#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m::asm;
use cortex_m_rt::entry;

// Константа с адресом загрузчика
const LOADER_ADDR: u32 = 0x08004000;

#[entry]
fn main() -> ! {
    // В реальной программе здесь был бы код для получения и установки новой прошивки
    // В минимальном варианте просто бесконечный цикл
    
    // Для имитации финальной части обновления, мы можем прыгнуть обратно в loader
    // Но для минимальной демонстрации просто зацикливаемся
    loop {
        asm::nop();
    }
}

// Функция, которую бы вызвали в полной версии при завершении обновления
#[allow(dead_code)]
fn jump_back_to_loader() -> ! {
    unsafe {
        // Отключаем прерывания
        cortex_m::interrupt::disable();
        
        // Устанавливаем таблицу векторов
        let scb = cortex_m::peripheral::SCB::ptr();
        (*scb).vtor.write(LOADER_ADDR);
        
        // Устанавливаем указатель стека
        let stack_ptr = *(LOADER_ADDR as *const u32);
        cortex_m::register::msp::write(stack_ptr);
        
        // Получаем адрес функции сброса
        let reset_addr = LOADER_ADDR + 4;
        let reset_ptr = *(reset_addr as *const u32);
        
        // Переходим к функции
        let jump_fn: fn() -> ! = core::mem::transmute(reset_ptr);
        jump_fn();
    }
    
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