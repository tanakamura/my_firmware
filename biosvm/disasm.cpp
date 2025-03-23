#include "vm.hpp"

void disasm(const VM *vm, int mode) {
    auto &sregs = vm->cpu->sregs;
    auto &regs = vm->cpu->regs;

    printf("rip=%x, rsp=%x flags=%08x\n", (int)regs.rip,
           (int)regs.rsp,
           (int)regs.rflags);

    unsigned char *code;

    auto pc = regs.rip + sregs.cs.base;

    if (mode == MODE_SPIFLASH) {
        auto rom_start = (1ULL<<32) - 256*1024;
        auto rom_offset = pc - rom_start;
        code = &vm->rom[rom_offset];
    } else if (mode == MODE_SDRAM) {
        auto pc = regs.rip + sregs.cs.base;
        code = &vm->sdram[pc];
    } else if (mode == MODE_OPTIONROM) {
        auto pc = regs.rip + sregs.cs.base;
        code = &vm->sdram[pc];
    }
    if (code[0] == 0xf3 && code[1] == 0xa4) {
        puts("rep movsb");
    } else if (code[0] == 0xf3 && code[1] == 0xa5) {
        puts("rep movsw");
    } else if (code[0] == 0xf3 && code[1] == 0xab) {
        puts("rep stosw");
    } else {
        FILE *fp = fopen("out.bin", "wb");
        fwrite(code, 1, 16, fp);
        fclose(fp);

        fflush(stdout);
        int r = system("ndisasm -b 16 out.bin|head -n2");
        if (r != 0) {
            exit(1);
        }
        fflush(stdout);
    }


    fflush(stdout);
}
