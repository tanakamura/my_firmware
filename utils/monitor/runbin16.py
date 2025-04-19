import serial
from monitor import *
import monitor
import sys

MCHBASE = 0xf0000000


def do_ram_command(m, com):
    write32(m, MCHBASE+0x0200, com | 0x00400)
    v = read32(m, MCHBASE+0x0200)
    print(f"{v:x}")


def main():
    m, args = open_machine()

    init(m)

    binary = open(args.binary, "rb").read()

    loadbin16(m, binary)
    print("loaded")

    regs = [0] * 14
    regs[0] = 0 #eax
    regs[1] = 0 #ecx
    regs[2] = 0 #edx
    regs[3] = 0 #ebx
    regs[4] = 0xfffc #esp
    regs[5] = 0 #ebp
    regs[6] = 0 #esi
    regs[7] = 0 #edi

    regs[8] = 0 #eflags


    seg = 0x1000
    regs[9] = 0xf000 # es
    regs[10] = 0xf000 # ss
    regs[11] = 0 # eip
    regs[12] = 0xf000 # ds
    regs[13] = seg # cs

    r = runbin16(m, regs)

    print(f"{r:x}")


if __name__ == "__main__":
    main()
