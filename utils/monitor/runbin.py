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

    #outb(m, 0x3fa, 0x3fa)
    outb(m, 0x3fa, 0x7)
    fifo = inb(m, 0x3fa)
    print(f"fifo = {fifo:x}")

    binary = open(args.binary, "rb").read()

    loadbin32(m, binary)
    print("loaded")
    r = runbin32(m)

    print(f"{r:x}")


if __name__ == "__main__":
    main()
