OUTPUT_ARCH("i386")
OUTPUT_FORMAT("binary")
ENTRY(reset)


RAM_START = 512M; /* LOADBIN_BASE */
RAM_SIZE = 256M;
MEMORY {
       BDA (rw) : org = 0x400, len = 256
       SDRAM (rwx) : org = RAM_START, len = RAM_SIZE
}

SECTIONS {
         bda : {
                service_func_table = .;
                . += 16 * 4;
                state16_legs = .;
                . += 16 * 4;
                state32_esp = .;
                . += 4;
         } > BDA

         .text : {
               __start32 = .;
               *(.text.start);
               KEEP(*(.text.keep))
               *(.text)
               *(.text.*)
         } > SDRAM

         . = ALIGN(16);

         .rodata : {
               *(.rodata* .rdata* .rodata..*)
         } > SDRAM

         . = ALIGN(16);

         .data : {
               *(.data* .rwdata*)
         } > SDRAM

         . = ALIGN(16);
         .bss : {
              __BSS_RAM_START = .;
               *(.bss*)
               . = ALIGN(64);
               __end = .;
         } > SDRAM

         __ram_last = RAM_START + RAM_SIZE;
         __heap_size = __ram_last - __end;
         __stack_bottom = RAM_START + RAM_SIZE;

         __end16_in_segment = 64K;
         __end16_flat32 = 64K;
         service_func_table_flat32 = 0xf0000;
         state16_regs_flat32 = 0xf0000 + 16*4;

         /DISCARD/ : { *(*) }
}
