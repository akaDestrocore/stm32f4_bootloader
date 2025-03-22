#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m::asm;
use cortex_m_rt::entry;

// Структура версии прошивки
#[repr(C, packed)]
pub struct ImageVersion {
    pub signature0: u32,
    pub signature1: u8,
    pub version_major: u8,
    pub version_minor: u8,
    pub version_patch: u8,
}

// Версия приложения
#[link_section = ".image_ver"]
#[used]
static IMAGE_VERSION: ImageVersion = ImageVersion {
    signature0: 0xDABA,
    signature1: 0xF,
    version_major: 1,
    version_minor: 2,
    version_patch: 3,
};

#[entry]
fn main() -> ! {
    // Основная функциональность приложения
    // В минимальном варианте просто бесконечный цикл
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