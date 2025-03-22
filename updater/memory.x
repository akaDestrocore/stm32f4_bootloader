/* STM32F407VGTx - 1024K FLASH, 192K RAM (128K + 64K CCMRAM) */
MEMORY
{
  /* MainFLASH */
  FLASH  (rx)     : ORIGIN = 0x08008000,   LENGTH = 96K
  
  /* Main RAM */
  RAM    (rwx)    : ORIGIN = 0x20000000,   LENGTH = 128K
  
  /* CCM (Core Coupled Memory) RAM */
  CCMRAM (rwx)    : ORIGIN = 0x10000000,   LENGTH = 64K
}

/* Stack start */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);