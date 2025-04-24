import serial
from monitor import *
import monitor
import sys
import array
import time

MCHBASE = 0xf0000000


def do_ram_command(m, com):
    write32(m, MCHBASE+0x0200, com | 0x00400)
    v = read32(m, MCHBASE+0x0200)
    print(f"{v:x}")


def main():
    m, args = open_machine()

    init(m)

    #pci_config_write16(m, 0, 0x1e, 0, 0x3e, 0x40 | 0x1c)
    #time.sleep(1)
    #pci_config_write16(m, 0, 0x1e, 0, 0x3e, 0x1c)
    #time.sleep(1)

    pci_config_write16(m, 0, 0x1e, 0x0, 0x1c, 0x9000)
    base = pci_config_read16(m, 0, 0x1e, 0x0, 0x1c)
    print(f"{base:x}")
    pci_config_write16(m, 0x5, 0, 0, 0x4, 7)

    outb(m, 0x46e8, 0x0016)
    outb(m, 0x102, 0x001)
    outb(m, 0x46e8, 0x000e)

    x = inb(m, 0x3cc)
    print(f"{x:x}")

    outb(m, 0x46e8, 0x0026)
    outb(m, 0x102, 0x001)
    outb(m, 0x46e8, 0x000e)
    outb(m, 0x4ae8, 0x0000)

    x = inb(m, 0x3cc)
    print(f"{x:x}")

    outb(m, 0x3d4, 0x36)
    x = inb(m, 0x3d5)
    print(f"{x:x}")

if __name__ == "__main__":
    main()
