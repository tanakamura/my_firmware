OUTPUT_ARCH("i386")
OUTPUT_FORMAT("binary")
TARGET("binary")
ENTRY(reset)

ROM_FULL_SIZE = 256K;
ROM16_SIZE = 4K;
ROM32_SIZE = ROM_FULL_SIZE - ROM16_SIZE;

RAM_START = 256M;
RAM_SIZE = 128M;
ROM_START = 4096M - ROM_FULL_SIZE;

/*
 *            cpu addr space                spi flash
 *                                          addr space
 *
 *          0    +---------+
 *               |         |
 *               |         |
 *               |         |
 *               |         |
 *               |         |
 *               +---------+
 *               | ROM16   |
 *               | SIZE    |
 *               |         |
 * 0x0010_0000   +---------+
 *               |         |
 *               |         |
 *               |         |
 *  RAM_START    +---------+
 *               | RAM_SIZE|
 *               |         |
 *               |         |
 *               +---------+
 *               |         |
 *               |         |
 *  ROM_START    +---------+              0  +---------+ <- ROM_STORE_32
 *               | ROM32   |                 |  ROM32  |
 *               | SIZE    |                 |  SIZE   |
 *               |         |                 |         |
 *               |         |                 |         |
 *               |         |                 |         |
 *               |         |                 |         |
 *               |         |                 |         |
 *               +---------+                 +---------+ <- ROM_STORE_16
 *               | ROM16   |                 |  ROM16  |
 *               | SIZE    |                 |  SIZE   |
 * RESET_VECTOR->|         |                 |         | <- RESET_VECTOR_ROM
 * 0xffff_ffff   +---------+  ROM_FULL_SIZE  +---------+
 *
 */

__RAM16_SEGMENT = 0xF000;
__RAM16_START_FLAT32 = 0xF0000;

MEMORY {
       RAM (rwx) : org = RAM_START, len = RAM_SIZE

       ROM_32 (rx) : org = ROM_START, len = ROM32_SIZE
       ROM_16 (rx) : org = 64K - ROM16_SIZE, len = ROM16_SIZE -16
       RAM_16 (rx) : org = 0, len = 64K

       ROM_STORE_32 (rx) : org = 0, len = ROM32_SIZE
       ROM_STORE_16 (rx) : org = ROM32_SIZE, len = ROM16_SIZE - 16

       RESET_VECTOR (rx) : org = 4096M - 16, len = 16
       RESET_VECTOR_ROM (rx) : org = ROM_FULL_SIZE - 16, len = 16

       BDA (rw) : org = 0x400, len = 256
}

SECTIONS {
         bda : {
                service_func_table = .;
                . += 16 * 4;
                state16_regs = .;
                . += 16 * 4;
                state32_esp = .;
                . += 4;
                bda_end = 0x400 + 256;
         } > BDA

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
         __LOAD_ROM_START = LOADADDR(.main_region_data) + ROM_START;
         __LOAD_SIZE_DW = (__LOAD_RAM_END - __LOAD_RAM_START)/4;

         .main_region_zero : {
               __BSS_RAM_START = .;
               *(.bss .bss.*)
               . = ALIGN(16);
               __BSS_RAM_SIZE_DW = (. - __BSS_RAM_START) / 4;
               . = ALIGN(64);
               __end = .;
         } > RAM AT > ROM_STORE_32
         __ram_last = RAM_START + RAM_SIZE;
         __heap_size = __ram_last - __end;
         __stack_bottom = RAM_START + RAM_SIZE;


         .rom16 : {
               FILL(0xffffffff);
               KEEP(*(.text16.rom))
         } > ROM_16 AT > ROM_STORE_16

         /* F000:0000 */
         .ram16 : {
               FILL(0xffffffff);
               __LOAD_RAM16_START = .;
               __LOAD_RAM16_START_FLAT32 = . + __RAM16_START_FLAT32;
               KEEP(*(.text16.ram))
               KEEP(*(.rodata16.ram))
               KEEP(*(.data16.ram))
               . = ALIGN(16);
               __LOAD_RAM16_END = .;
         } > RAM_16 AT > ROM_STORE_32

         .main16_region_zero : {
               __BSS_RAM16_START_IN_SEGMENT = .;
               __BSS_RAM16_START_FLAT32 = . + __RAM16_START_FLAT32;

               *(.bss16)
               . = ALIGN(16);
               __BSS_RAM16_SIZE_DW = (. - __BSS_RAM16_START_IN_SEGMENT) / 4;
               __end16_in_segment = .;
               __end16_flat32 = . + __RAM16_START_FLAT32;
         } > RAM_16 AT > ROM_STORE_32
         __stack16_bottom_in_segment = 0;

         __LOAD_ROM16_START = LOADADDR(.ram16) + ROM_START;
         __LOAD_ROM16_SIZE_DW = (__LOAD_RAM16_END - __LOAD_RAM16_START)/4;

         .reset_vector : {
               KEEP(*(.reset16))
               . = ALIGN(16);
         } > RESET_VECTOR AT > RESET_VECTOR_ROM

         /DISCARD/ : { *(.note.* .eh_frame .comment ) }
}


