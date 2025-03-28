#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f4 as pac;


#[entry]
fn main() -> ! {
    // PD12 init 
    let peripherals: stm32f4::Peripherals = unsafe { pac::Peripherals::steal() };
    
    // Enable GPIOD clock
    unsafe {
        peripherals.rcc.ahb1enr().modify(|_, w| w.gpioden().set_bit());

        peripherals.gpiod.moder().modify(|_, w| w.moder12().bits(0b01));
        peripherals.gpiod.otyper().modify(|_, w| w.ot12().clear_bit());
        peripherals.gpiod.ospeedr().modify(|_, w| w.ospeedr12().bits(0b00));
        
        // enable LED
        peripherals.gpiod.bsrr().write(|w| w.bs12().set_bit());
    }

    
    // just blink
    loop {
        toggle_led(&peripherals);
        delay_ms(500);
    }
}

fn toggle_led(peripherals: &pac::Peripherals) {
    unsafe {
        let current: bool = peripherals.gpiod.odr().read().odr12().bit();
        if current {
            peripherals.gpiod.bsrr().write(|w| w.br12().set_bit());
        } else {
            peripherals.gpiod.bsrr().write(|w| w.bs12().set_bit());
        }
    }
}

fn delay_ms(ms: u32) {
    let mut count: u32 = ms * 8000; 
    while count > 0 {
        count -= 1;
        asm::nop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        asm::nop();
    }
}