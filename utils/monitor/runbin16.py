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

    regs[8] = 0 #eflags, EI=1

    seg = 0x1000
    regs[9] = seg # es
    regs[10] = seg # ss
    regs[11] = 0 # eip
    regs[12] = seg # ds
    regs[13] = seg # cs

    regs = runbin16(m, regs)

    print(f"eax:{regs[0]:08x}")
    print(f"ecx:{regs[1]:08x}")
    print(f"edx:{regs[2]:08x}")
    print(f"ebx:{regs[3]:08x}")
    print(f"esp:{regs[4]:08x}")
    print(f"ebp:{regs[5]:08x}")
    print(f"esi:{regs[6]:08x}")
    print(f"edi:{regs[7]:08x}")
    print(f"eflags:{regs[8]:08x}")
    print(f"es:{regs[9]:08x}")
    print(f"ss:{regs[10]:08x}")
    print(f"eip:{regs[11]:08x}")
    print(f"ds:{regs[12]:08x}")
    print(f"cs:{regs[13]:08x}")


if __name__ == "__main__":
    main()
