#![allow(unused)]
use cortex_m_rt::{exception, ExceptionFrame};

// Only include these exception handlers when the "exceptions" feature is enabled
#[cfg(feature = "exceptions")]
#[exception]
unsafe fn NonMaskableInt() {
    loop {}
}

#[cfg(feature = "exceptions")]
#[exception]
unsafe fn HardFault(_ef: &ExceptionFrame) -> ! {
    loop {}
}

#[cfg(feature = "exceptions")]
#[exception]
unsafe fn MemoryManagement() {
    loop {}
}

#[cfg(feature = "exceptions")]
#[exception]
unsafe fn BusFault() {
    loop {}
}

#[cfg(feature = "exceptions")]
#[exception]
unsafe fn UsageFault() {
    loop {}
}

#[cfg(feature = "exceptions")]
#[exception]
unsafe fn SVCall() {
    loop {}
}

#[cfg(feature = "exceptions")]
#[exception]
unsafe fn DebugMonitor() {
    loop {}
}

#[cfg(feature = "exceptions")]
#[exception]
unsafe fn PendSV() {
    loop {}
}

// SysTick is useful to always have
#[exception]
unsafe fn SysTick() {
    crate::stm32f4_systick::SysTick::increment_tick();
}