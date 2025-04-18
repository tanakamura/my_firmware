OUTPUT_ARCH("i386")
OUTPUT_FORMAT("binary")
ENTRY(reset)


__RAM16_SEGMENT = 0xF000;
__RAM16_START_FLAT32 = 0xF0000;

RAM_START = 512M; /* LOADBIN_BASE */
RAM_SIZE = 256M;
MEMORY {
       REAL_SEGMENT_REGION (rw) : org = __RAM16_START_FLAT32, len = 256
       SDRAM (rwx) : org = RAM_START, len = RAM_SIZE
}

SECTIONS {
         real_segment_region : {
                service_func_table_in_segment = .;
                service_func_table_flat32 = service_func_table_in_segment + __RAM16_START_FLAT32;
                . += 16 * 8;
                state16_regs_in_segment = .;
                state16_regs_flat32 = state16_regs_in_segment + __RAM16_START_FLAT32;
                . += 16 * 4;
                state32_esp_in_segment = .;
                state32_esp_flat32 = state32_esp_in_segment + __RAM16_START_FLAT32;
                . += 4;
         } > REAL_SEGMENT_REGION

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
