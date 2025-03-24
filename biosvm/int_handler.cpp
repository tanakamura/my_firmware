#include "vm.hpp"

void install_int_handler_readlmode(VM *vm,
                                   std::function<void(VM *vm, CPU *cpu)> f,
                                   int num) {
    uint16_t *p16 = (uint16_t *)(&vm->sdram[num * 4]);

    p16[0] = num;
    p16[1] = 0xf000;

    uint8_t *p8 = (uint8_t *)(&vm->sdram[0xf0000 + num]);

    /* fill by hlt */
    p8[0] = 0xf4;

    vm->int_handlers[num] = f;
}
