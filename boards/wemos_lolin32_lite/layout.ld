MEMORY
{
  rom (rx)  : ORIGIN = 0x3F400000, LENGTH = 0x30000
  ram (rwx) : ORIGIN = 0x3F800000, LENGTH = 0x7EF40
  prog (rx) : ORIGIN = 0x3F430020, LENGTH = 0x3A08E0
}

INCLUDE tock_kernel_layout.ld
