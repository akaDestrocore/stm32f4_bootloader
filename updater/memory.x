MEMORY
{
  FLASH : ORIGIN = 0x08008000, LENGTH = 96K
  RAM : ORIGIN = 0x20000000, LENGTH = 128K
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);