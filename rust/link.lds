OUTPUT_ARCH("i386")
OUTPUT_FORMAT("binary")
ENTRY(reset)

ROM_FULL_SIZE = 256K;
ROM16_SIZE = 4K;
ROM32_SIZE = ROM_FULL_SIZE - ROM16_SIZE;
ROM_OFFSET = 4096M - ROM_FULL_SIZE;

RAM_START = 0x00400000;
RAM_SIZE = 128M;
ROM_START = 4096M - ROM_FULL_SIZE;

MEMORY {
       RAM (rwx) : org = RAM_START, len = RAM_SIZE

       ROM_32 (rx) : org = ROM_START, len = ROM32_SIZE
       ROM_16 (rx) : org = 64K - ROM16_SIZE, len = ROM16_SIZE -16

       ROM_STORE_32 (rx) : org = 0, len = ROM32_SIZE
       ROM_STORE_16 (rx) : org = ROM32_SIZE, len = ROM16_SIZE - 16

       RESET_VECTOR (rx) : org = 4096M - 16, len = 16
       RESET_VECTOR_ROM (rx) : org = ROM_FULL_SIZE - 16, len = 16
}

SECTIONS {
         .init_region : {
               FILL(0xffffffff);
               . = 0;
               KEEP(*(.text.init32))
               KEEP(*(.rodata.init32))
         } > ROM_32 AT > ROM_STORE_32

         .main_region_data : {
               FILL(0xffffffff);
               __LOAD_RAM_START = .;
               *(.text)
               *(.text.*)
               *(.rodata* .rdata* .rodata..*)
               *(.data* .rwdata*)
               . = ALIGN(16);
               __LOAD_RAM_END = .;
         } > RAM AT > ROM_STORE_32
         __LOAD_ROM_START = LOADADDR(.main_region_data) + ROM_OFFSET;
         __LOAD_SIZE_DW = (__LOAD_RAM_END - __LOAD_RAM_START)/4;

         .main_region_zero : {
               __BSS_RAM_START = .;
               *(.bss)
               . = ALIGN(16);
               __BSS_RAM_SIZE_DW = (. - __BSS_RAM_START) / 4;
         } > RAM AT > ROM_STORE_32
         __stack_bottom = RAM_START + RAM_SIZE;


         .rom16 : {
               FILL(0xffffffff);
               KEEP(*(.text16))
         } > ROM_16 AT > ROM_STORE_16

         .reset_vector : {
               KEEP(*(.reset16))
               . = ALIGN(16);
         } > RESET_VECTOR AT > RESET_VECTOR_ROM

         /DISCARD/ : { *(.note.* .eh_frame .comment ) }
}


