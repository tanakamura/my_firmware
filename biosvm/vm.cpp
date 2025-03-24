#include "vm.hpp"

void VM::emu_reti() {
    auto &regs = cpu->regs;
    auto &sregs = cpu->sregs;
    uint16_t prev_ip = *(uint16_t *)(sdram + (sregs.ss.base + regs.rsp));
    uint16_t prev_cs = *(uint16_t *)(sdram + (sregs.ss.base + regs.rsp + 2));
    uint16_t prev_flags =
        *(uint16_t *)(sdram + (sregs.ss.base + regs.rsp + 4));

    sregs.cs.base = prev_cs * 16;
    sregs.cs.selector = prev_cs;
    regs.rflags = prev_flags;
    regs.rip = prev_ip;
    regs.rsp += 6;
}

void VM::emu_far_ret() {
    auto &regs = cpu->regs;
    auto &sregs = cpu->sregs;

    uint16_t prev_ip = *(uint16_t *)(sdram + (sregs.ss.base + regs.rsp));
    uint16_t prev_cs = *(uint16_t *)(sdram + (sregs.ss.base + regs.rsp + 2));

    sregs.cs.base = prev_cs * 16;
    sregs.cs.selector = prev_cs;

    regs.rip = prev_ip;
    regs.rsp += 4;
}

void VM::emu_push16(uint16_t val) {
    auto &regs = cpu->regs;
    auto &sregs = cpu->sregs;

    regs.rsp -= 2;
    auto dest = (uint16_t *)(sdram + sregs.ss.base + regs.rsp);

    *dest = val;
}

inline void set_seg(struct kvm_segment &sreg, uintptr_t sel_val) {
    sreg.base = sel_val * 16;
    sreg.limit = 0xffff;
    sreg.selector = sel_val;
    sreg.db = 0;
    sreg.l = 0;
}

//void VM::emu_far_call(uintptr_t cs, uintptr_t ip) {
//    emu_push16(0xf000);                      // bios cs
//    emu_push16(0x200);                       // ret intr
//
//    set_seg(cpu->sregs.cs, cs);
//    cpu->regs.rip = ip;
//
//    run_with_handler(this);
//}
