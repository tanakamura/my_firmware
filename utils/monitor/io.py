import serial
from monitor import *
import monitor
import sys
import array

MCHBASE = 0xf0000000


def do_ram_command(m, com):
    write32(m, MCHBASE+0x0200, com | 0x00400)
    v = read32(m, MCHBASE+0x0200)
    print(f"{v:x}")


def main():
    m, args = open_machine()

    init(m)

#    data = array.array('I')
#
#    for i in range(0xd400//4):
#        v = read32(m, 0xc0000 + i*4)
#        print(f"{v:x}")
#        data.append(v)
#
#    f = open('vgabios_dumped.bin', 'wb')
#    f.write(data.tobytes())
#    f.close()
    if True:
        outb(m, 0x20, 0x11) # ICW1
        outb(m, 0xa0, 0x11) # ICW1
        outb(m, 0x21, 0x00) # ICW2
        outb(m, 0xa1, 0x08) # ICW2, offset=8
        outb(m, 0x21, 0x04) # ICW3, 2=slave
        outb(m, 0xa1, 0x02) # ICW3, cascade to 2
        outb(m, 0x21, 0x01) # ICW4, 8086
        outb(m, 0xa1, 0x01) # ICW4, 8086

        outb(m, 0x21, 0xfe) # OCW1, enable irq 0
        outb(m, 0xa1, 0xff) # OCW1, disable all

        outb(m, 0x43, 0x34)
        outb(m, 0x40, 0)
        outb(m, 0x40, 0)

    pit_val = inb(m, 0x40)

    for i in [0x20, 0x21, 0xa0, 0xa1]:
        pic_val = inb(m, i)
        print(f"PIC: {pic_val:x}")

    outb(m, 0x40, 0) # latch value

    lo = inb(m, 0x40)
    hi = inb(m, 0x40)
    pit_val = (hi << 8) | lo
    print(f"PIT: {pit_val:x}")


if __name__ == "__main__":
    main()
