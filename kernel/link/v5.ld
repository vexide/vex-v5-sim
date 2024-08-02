OUTPUT_FORMAT("elf32-littlearm")

ENTRY(reset)

_low_memory_start = 0x0;
_user_memory_start = 0x03800000;
_jump_table_start = _user_memory_start - 0x4000;

MEMORY {
  low_mem : ORIGIN = _low_memory_start, LENGTH = _jump_table_start - _low_memory_start
  jump_table_mem (rx) : ORIGIN = _jump_table_start, LENGTH = _user_memory_start - _jump_table_start
}

__stack_length = 0x400000;
/*
This arbitrary subtraction was pulled from vexide's linker script. The heap is big enough that it
probably won't be noticeable so I played it safe by keeping the same value.
*/
__heap_end = _jump_table_start - __stack_length - 0x100;

SECTIONS {
    _vex_startup = _user_memory_start + 0x20;

    .text ALIGN(4096) : {
        __text_start = .;
        KEEP (*(.text.vectors))
        *(.text.boot);
        *(.text .text*)
    } > low_mem

    .rodata ALIGN(4096) : {
        *(.rodata*)
    } > low_mem

    .data ALIGN(4096) : {
        *(.data*)
    } > low_mem

    .bss ALIGN(4096) : {
        *(.bss*)
    } > low_mem

    .heap (NOLOAD) : ALIGN(4) {
        __heap_start = .;
        . = __heap_end;
    } > low_mem

    .stack (NOLOAD) : ALIGN(8) {
        __stack_bottom = .;
        . += __stack_length;
        __stack_top = .;
    } > low_mem

    .jump_table : {
        KEEP(*(.jump_table))
    } > jump_table_mem
}
