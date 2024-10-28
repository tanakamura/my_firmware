OUTPUT_ARCH("i386")
OUTPUT_FORMAT("binary")
ENTRY(reset)


SECTIONS {
         . = 0x10000000;

         .text : {
               __start32 = .;
               *(.text.start);
               KEEP(*(.text.keep))
               *(.text)
               *(.text.*)
         }

         . = ALIGN(16);

         .rodata : {
               *(.rodata* .rdata* .rodata..*)
         }

         . = ALIGN(16);

         .data : {
               *(.data* .rwdata*)
         }

         . = ALIGN(16);
         .bss : {
               *(.bss*)
          }

         /DISCARD/ : { *(*) }
}
