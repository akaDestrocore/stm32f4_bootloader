#![no_std]
#![no_main]

include!(concat!(env!("OUT_DIR"), "/header_values.rs"));

use core::panic::PanicInfo;
use cortex_m::{asm, peripheral::SYST};
use cortex_m_rt::{entry, exception};
use stm32f4 as pac;
use misc::{
    systick,
    uart::UartManager,
    led::Leds,
};

// symbol from linker
extern "C" {
    static __firmware_size: u32;
}

// animation frames for serial terminal
const FRAMES: [&str; 11] = [
    "+----------------------+\r\n\
     |                      |\r\n\
     |  |                |  |\r\n\
     |  | o              |  |\r\n\
     |  |                |  |\r\n\
     |  |                |  |\r\n\
     |                      |\r\n\
     +----------------------+\r\n",
    
    "+----------------------+\r\n\
     |                      |\r\n\
     |  |                |  |\r\n\
     |  |                |  |\r\n\
     |  |    o           |  |\r\n\
     |  |                |  |\r\n\
     |                      |\r\n\
     +----------------------+\r\n",
    
    "+----------------------+\r\n\
     |                      |\r\n\
     |  |                |  |\r\n\
     |  |                |  |\r\n\
     |  |                |  |\r\n\
     |  |       o        |  |\r\n\
     |                      |\r\n\
     +----------------------+\r\n",
    
    "+----------------------+\r\n\
     |                      |\r\n\
     |  |                |  |\r\n\
     |  |                |  |\r\n\
     |  |          o     |  |\r\n\
     |  |                |  |\r\n\
     |                      |\r\n\
     +----------------------+\r\n",
    
    "+----------------------+\r\n\
     |                      |\r\n\
     |  |                |  |\r\n\
     |  |             o  |  |\r\n\
     |  |                |  |\r\n\
     |  |                |  |\r\n\
     |                      |\r\n\
     +----------------------+\r\n",
    
    "+----------------------+\r\n\
     |                      |\r\n\
     |  |                |  |\r\n\
     |  |               o|  |\r\n\
     |  |                |  |\r\n\
     |  |                |  |\r\n\
     |                      |\r\n\
     +----------------------+\r\n",
    
    "+----------------------+\r\n\
     |                      |\r\n\
     |  |                |  |\r\n\
     |  |                |  |\r\n\
     |  |            o   |  |\r\n\
     |  |                |  |\r\n\
     |                      |\r\n\
     +----------------------+\r\n",
    
    "+----------------------+\r\n\
     |                      |\r\n\
     |  |                |  |\r\n\
     |  |                |  |\r\n\
     |  |                |  |\r\n\
     |  |        o       |  |\r\n\
     |                      |\r\n\
     +----------------------+\r\n",
    
    "+----------------------+\r\n\
     |                      |\r\n\
     |  |                |  |\r\n\
     |  |                |  |\r\n\
     |  |     o          |  |\r\n\
     |  |                |  |\r\n\
     |                      |\r\n\
     +----------------------+\r\n",
    
    "+----------------------+\r\n\
     |                      |\r\n\
     |  |                |  |\r\n\
     |  | o              |  |\r\n\
     |  |                |  |\r\n\
     |  |                |  |\r\n\
     |                      |\r\n\
     +----------------------+\r\n",

    "+----------------------+\r\n\
     |                      |\r\n\
     |  |                |  |\r\n\
     |  |o               |  |\r\n\
     |  |                |  |\r\n\
     |  |                |  |\r\n\
     |                      |\r\n\
     +----------------------+\r\n",
];

// animation banner
const BOOT_BANNER: &str = "\r\n\
\x1B[36mxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\r
xxxxxxxx  xxxxxxxxxxxxxxxxxxxx  xxxxxxxxx\r
xxxxxxxxxx  xxxxxxxxxxxxxxxxx  xxxxxxxxxx\r
xxxxxx  xxx  xxxxxxxxxxxxxxx  xx   xxxxxx\r
xxxxxxxx  xx  xxxxxxxxxxxxx  xx  xxxxxxxx\r
xxxx  xxx   xxxxxxxxxxxxxxxxx  xxx  xxxxx\r
xxxxxx    xxxx  xxxxxxxx  xxx     xxxxxxx\r
xxxxxxxx xxxxx xx      xx xxxx  xxxxxxxxx\r
xxxx     xxxxx   xx  xx   xxxxx     xxxxx\r
xxxxxxxx xxxxxxxxxx  xxxxxxxxxx  xxxxxxxx\r
xxxxx    xxxxxx  xx  xx  xxxxxx    xxxxxx\r
xxxxxxxx  xxxx xxxx  xxxx xxxxx xxxxxxxxx\r
xxxxxxx    xxx  xxx  xxx  xxx    xxxxxxxx\r
xxxxxxxxxx   xxxxxx  xxxxxx   xxxxxxxxxxx\r
xxxxxxxxxxxxxx             xxxxxxxxxxxxxx\r
xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\r\n\x1B[0m\r\n\
\x1B[33m-- Firmware v1.0.0 --\x1B[0m\r\n\
\r\n";

#[entry]
fn main() -> ! {
    // get size from linker
    let firmware_size: u32 = unsafe {
        let size: *const u32 = &__firmware_size as *const u32;
        let size_value: u32 = *size;
        
        // update size data in header
        IMAGE_HEADER.update_data_size(size_value);
        
        size_value
    };

    let p: pac::Peripherals = pac::Peripherals::take().unwrap();
    let mut cp: cortex_m::Peripherals = cortex_m::Peripherals::take().unwrap();
    
    // Configure system clock
    setup_system_clock(&p);
    
    // Enable peripherals
    enable_peripherals(&p);
    
    // Configgure GPIO ports
    setup_gpio_pins(&p);
    
    // Configure SysTick
    systick::setup_systick(&mut cp.SYST);
    
    // imit UART and on-board LED pins
    let mut leds: Leds<'_> = Leds::new(&p);
    let mut uart: UartManager<'_> = UartManager::new(&p);
    
    leds.init();
    uart.init();
    
    // Enable USART2 NVIC interrupt (needed for ring buffer to work properly)
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::USART2);
    }

    clear_screen(&mut uart);
    
    // print banner
    uart.send_string(BOOT_BANNER);
    delay_ms(2000);
    
    // animation loop
    let mut frame_index: usize = 0;
    let mut led_index: u8 = 0;
    let mut last_update: u32 = systick::get_tick_ms();
    let mut last_led_toggle: u32 = systick::get_tick_ms();
    
    loop {
        // update UART
        uart.process();
        
        // get current time from SysTick
        let current_time: u32 = systick::get_tick_ms();
        
        // every 200 ms
        if current_time.wrapping_sub(last_update) >= 200 {
            // clear screen and reset cursor position
            uart.send_string("\x1B[2J\x1B[1;1H");
            
            // Send colored title
            uart.send_string("\x1B[96mPONG ANIMATION\x1B[0m\r\n\r\n");
            
            // send current frame in blue color
            uart.send_string("\x1B[34m");
            uart.send_string(FRAMES[frame_index]);
            uart.send_string("\x1B[0m");
            
            // additional info
            uart.send_string("\r\n\x1B[91mFirmware v1.0.0 - System uptime: ");
            uart.send_string(itoa(current_time / 1000));
            uart.send_string(" sec\x1B[0m\r\n");
            
            // next frame
            frame_index = (frame_index + 1) % FRAMES.len();
            last_update = current_time;
        }
        
        // also blink leds for 100 ms each
        if current_time.wrapping_sub(last_led_toggle) >= 100 {
            // first turn off all of them
            for i in 0..4 {
                leds.set(i, false);
            }
            
            // enable next in queue
            leds.set(led_index, true);
            
            // move to next one
            led_index = (led_index + 1) % 4;
            last_led_toggle = current_time;
        }
    }
}

fn itoa(mut value: u32) -> &'static str {
    static mut BUFFER: [u8; 16] = [0; 16];
    
    if value == 0 {
        return "0";
    }
    
    let mut i: usize = 0;
    unsafe {
        while value > 0 && i < BUFFER.len() {
            BUFFER[i] = b'0' + (value % 10) as u8;
            value /= 10;
            i += 1;
        }
        
        // Reverse the digits
        let mut j: usize = 0;
        let mut k: usize = i - 1;
        while j < k {
            let temp: u8 = BUFFER[j];
            BUFFER[j] = BUFFER[k];
            BUFFER[k] = temp;
            j += 1;
            k -= 1;
        }
        
        BUFFER[i] = 0;
        
        // convert to string
        core::str::from_utf8_unchecked(&BUFFER[0..i])
    }
}

fn clear_screen(uart: &mut UartManager) {
    uart.send_string("\x1B[2J\x1B[1;1H");
    for _ in 0..5 {
        uart.process();
    }
}

fn delay_ms(ms: u32) {
    let start: u32 = systick::get_tick_ms();
    while systick::get_tick_ms().wrapping_sub(start) < ms {
        asm::nop();
    }
}

fn setup_system_clock(p: &pac::Peripherals) {
    // Enable PWR clock
    p.rcc.apb1enr().modify(|_, w| w.pwren().set_bit());

    p.pwr.cr().modify(|_, w| w.vos().scale1());

    // flash latency 5 wait states
    p.flash.acr().modify(|_, w| w
        .latency().ws5()
        .prften().set_bit()
        .icen().set_bit()
        .dcen().set_bit()
    );

    // enable HSE
    p.rcc.cr().modify(|_, w| w.hseon().set_bit());
    while p.rcc.cr().read().hserdy().bit_is_clear() {
        // wait
    }

    // Set PLL for 90 Hz
    p.rcc.pllcfgr().modify(|_, w| unsafe {
        w.pllsrc().hse()
         .pllm().bits(4)
         .plln().bits(90)
         .pllp().div2()
         .pllq().bits(4)
    });

    // Enable PLL
    p.rcc.cr().modify(|_, w| w.pllon().set_bit());
    while p.rcc.cr().read().pllrdy().bit_is_clear() {
        // Ждем готовности PLL
    }

    // bus dividers
    p.rcc.cfgr().modify(|_, w| w
        .hpre().div1()
        .ppre1().div4()
        .ppre2().div2()
    );

    // set PLL as system clock
    p.rcc.cfgr().modify(|_, w| w.sw().pll());
    while !p.rcc.cfgr().read().sws().is_pll() {
        // wait
    }
}

fn enable_peripherals(p: &pac::Peripherals) {
    // enable GPIO clock
    p.rcc.ahb1enr().modify(|_, w| {
        w.gpioaen().enabled()
         .gpioden().enabled()
    });
    
    // enbale USART2 clock
    p.rcc.apb1enr().modify(|_, w| {
        w.usart2en().enabled()
    });
    
    // enbale SYSCFG
    p.rcc.apb2enr().modify(|_, w| {
        w.syscfgen().enabled()
    });
}

fn setup_gpio_pins(p: &pac::Peripherals) {
    // PA2 = TX, PA3 = RX
    p.gpioa.moder().modify(|_, w| {
        w.moder2().alternate()
         .moder3().alternate()
    });

    p.gpioa.afrl().modify(|_, w| {
        w.afrl2().af7()
         .afrl3().af7()
    });
    
    p.gpioa.ospeedr().modify(|_, w| {
        w.ospeedr2().high_speed()
         .ospeedr3().high_speed()
    });
    
    p.gpioa.pupdr().modify(|_, w| {
        w.pupdr2().floating()
         .pupdr3().floating()
    });
}

#[exception]
fn SysTick() {
    systick::increment_tick();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        asm::nop();
    }
}