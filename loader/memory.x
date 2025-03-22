/* STM32F407VGTx - 1024K FLASH, 192K RAM (128K + 64K CCMRAM) */
MEMORY
{
  /* MainFLASH */
  FLASH  (rx)     : ORIGIN = 0x08004000,   LENGTH = 16K
  
  /* Main RAM */
  RAM    (rwx)    : ORIGIN = 0x20000000,   LENGTH = 128K
  
  /* CCM (Core Coupled Memory) RAM */
  CCMRAM (rwx)    : ORIGIN = 0x10000000,   LENGTH = 64K
}

/* Stack start */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

/* Output file sections */
SECTIONS
{
  /* IRQ Vector table */
  .vector_table ORIGIN(FLASH) :
  {
    /* Stack beginning value */
    LONG(_stack_start);
    
    /* Cortex-M Vector Table */
    KEEP(*(.vector_table.reset_vector));
    KEEP(*(.vector_table.exceptions));
    
    /* STM32 IRQ Table */
    KEEP(*(.vector_table.interrupts));
    . = ALIGN(4);
  } > FLASH

  /* Code section */
  .text :
  {
    . = ALIGN(4);
    *(.text .text.*);
    . = ALIGN(4);
    KEEP(*(.init));
    KEEP(*(.fini));
    . = ALIGN(4);
  } > FLASH

  /* Const section */
  .rodata :
  {
    . = ALIGN(4);
    *(.rodata .rodata.*);
    . = ALIGN(4);
  } > FLASH

  /* Init section */
  .data : 
  {
    . = ALIGN(4);
    __sdata = .;        /* .data */
    *(.data .data.*);
    . = ALIGN(4);
    __edata = .;        /* end of .data */
  } > RAM AT> FLASH
  
  /* FLASH addr with init values */
  __sidata = LOADADDR(.data);

  /* bss */
  .bss :
  {
    . = ALIGN(4);
    __sbss = .;         /* .bss */
    *(.bss .bss.*);
    *(COMMON);
    . = ALIGN(4);
    __ebss = .;         /* end of .bss */
  } > RAM
  
  /* CCM RAM data */
  .ccmram :
  {
    . = ALIGN(4);
    __sccmram = .;      /* .ccmram */
    *(.ccmram .ccmram.*);
    . = ALIGN(4);
    __eccmram = .;      /* end of .ccmram */
  } > CCMRAM
}

/* reset handler */
ENTRY(Reset_Handler);