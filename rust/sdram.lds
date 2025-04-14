OUTPUT_ARCH("i386")
OUTPUT_FORMAT("binary")
ENTRY(reset)


MEMORY {
       BDA (rw) : org = 0x400, len = 256
       SDRAM (rwx) : org = 0x10000000, len = 256M
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
               *(.bss*)
         } > SDRAM

         /DISCARD/ : { *(*) }
}
