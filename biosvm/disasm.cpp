#include "vm.hpp"

void disasm(const VM *vm) {
    auto &sregs = vm->cpu->sregs;
    auto &regs = vm->cpu->regs;
    auto rom = vm->rom;

    printf("rip=%x, cs=%x, ds=%x ss=%x flags=%08x\n", (int)regs.rip,
           (int)sregs.cs.base, (int)sregs.ds.base, (int)sregs.ss.base, (int)regs.rflags);

    auto rom_start = (1ULL<<32) - 256*1024;
    auto pc = regs.rip + sregs.cs.base;
    auto rom_offset = pc - rom_start;
    unsigned char *code = &rom[rom_offset];
    if (code[0] == 0xf3 && code[1] == 0xa4) {
        puts("rep movsb");
    } else if (code[0] == 0xf3 && code[1] == 0xa5) {
        puts("rep movsw");
    } else if (code[0] == 0xf3 && code[1] == 0xab) {
        puts("rep stosw");
    } else {
        FILE *fp = fopen("out.bin", "wb");
        fwrite(&rom[rom_offset], 1, 16, fp);
        fclose(fp);

        fflush(stdout);
        int r = system("ndisasm -b 32 out.bin|head -n2");
        if (r != 0) {
            exit(1);
        }
        fflush(stdout);
    }


    fflush(stdout);
}
