use cortex_m::interrupt;

pub const INTERRUPT_COUNT: usize = 240;

#[unsafe(link_section = ".vector_table.interrupts")]
#[used]
pub static __INTERRUPTS: [Option<unsafe extern "C" fn()>; INTERRUPT_COUNT] = [None; INTERRUPT_COUNT];