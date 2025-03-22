/* STM32F407VGTx - 1024K FLASH, 192K RAM (128K + 64K CCMRAM) */
MEMORY
{
  /* MainFLASH */
  FLASH  (rx)     : ORIGIN = 0x08020000,   LENGTH = 512K
  
  /* Main RAM */
  RAM    (rwx)    : ORIGIN = 0x20000000,   LENGTH = 128K
  
  /* CCM (Core Coupled Memory) RAM */
  CCMRAM (rwx)    : ORIGIN = 0x10000000,   LENGTH = 64K
}

/* Stack start */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS {
    .image_ver (NOLOAD) : {
        KEEP(*(.image_ver))
    } > FLASH
}