
/* RAM is 240K because 16K are reserved for the bootloader */

MEMORY
{
  RAM   : ORIGIN = 0x20004000, LENGTH = 240K
}

SECTIONS
{
  .vector_table :
  {
    LONG(ORIGIN(RAM) + LENGTH(RAM))
    KEEP(*(.reset))
  } > RAM

  .text :
  {
    *(.text*)
  } > RAM

  /DISCARD/ :
  {
    *(.ARM.exidx*)
    *(.note.gnu.build-id*)
  }
}
