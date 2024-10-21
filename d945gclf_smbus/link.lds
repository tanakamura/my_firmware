OUTPUT_ARCH("i386")
OUTPUT_FORMAT("binary")

ROM_SIZE = 256K;
INIT16_SIZE = 128;

MEMORY {
       INIT16 (rx) : org = 64K-INIT16_SIZE, len = INIT16_SIZE
       ROM32 (rx) : org = 4096M-ROM_SIZE, len = (ROM_SIZE-INIT16_SIZE)
       ROM (rx) : org = 0, len = ROM_SIZE
}

SECTIONS {
         .text : {
               FILL(0xffffffff);
               __start32 = .;
               *(.text .text.*);
               *(.rodata* .rdata*);
               *(.data* .rwdata*);
               . = ALIGN(16);
               __size = .-__start32;
               . = .+(ROM_SIZE-INIT16_SIZE-__size);
               __stack_bottom = .;
         } > ROM32 AT > ROM

         .text16 : {
               FILL(0xffffffff);
               *(.text16);
               . = INIT16_SIZE-16;
               *(.reset16)
               . = ALIGN(16);
         } > INIT16 AT > ROM

         /DISCARD/ : { *(*) }
}
