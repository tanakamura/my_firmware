OUTPUT_FORMAT("binary")

ROM_SIZE = 64K;
INIT16_SIZE = 1K;

MEMORY {
       INIT16 (rx) : org = 64K-INIT16_SIZE, len = INIT16_SIZE
       ROM32 (rx) : org = 4096M-ROM_SIZE, len = (ROM_SIZE-INIT16_SIZE)
       ROM (rx) : org = 0, len = ROM_SIZE
}

SECTIONS {
         .text : {
               __start32 = .;
               *(.text);
               *(.rodata);
               . = ALIGN(16);
               __size = .-__start32;
               . = .+(ROM_SIZE-INIT16_SIZE-__size);
         } > ROM32 AT > ROM

         .text16 : {
               *(.text16);
               FILL(0x90909090);
               . = INIT16_SIZE-16;
               *(.reset16)
               FILL(0x90909090);
               . = ALIGN(16);
         } > INIT16 AT > ROM

         /DISCARD/ : { *(*) }
}
