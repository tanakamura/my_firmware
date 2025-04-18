import sys
import subprocess
import atexit
import readline
import struct
import serial

from monitor import *

def main():
    m, args = open_machine()

    init(m)

    oprom = read32(m, 0xc0000)
    print(f"OPROM: {oprom:x}")

    v3cc = inb(m, 0x3cc)
    print(f"V3CC: {v3cc:x}")

    #r = pci_config_read16(m, 0, 0, 0, 0x52)
    #print(f"GGC: {r:x}")
    ##disable IGD
    #pci_config_write16(m, 0, 0, 0, 0x52, 0)

    def assign_bus(m, cur_bus):
        bus = cur_bus
        for dev in range(32):
            vid = pci_config_read16(m, cur_bus, dev, 0, 0)
            if vid == 0xffff:
                continue
            headertyp = pci_config_read8(m, cur_bus, dev, 0, 0x0e)

            if headertyp & 0x80:
                num_func = 8
            else:
                num_func = 1

            for func in range(num_func):
                vid = pci_config_read16(m, cur_bus, dev, func, 0)
                if vid == 0xffff:
                    continue

                did = pci_config_read16(m, cur_bus, dev, func, 2)
                pi = pci_config_read8(m, cur_bus, dev, func, 0x09)
                scc = pci_config_read8(m, cur_bus, dev, func, 0x0a)
                cc = pci_config_read8(m, cur_bus, dev, func, 0x0b)

                print(f"{cur_bus:02x}:{dev:02x}:{func:02x} {vid:04x} {did:04x} {pi:02x} {scc:02x} {cc:02x}")

                cmd = pci_config_read16(m, cur_bus, dev, func, 0x04)
                pci_config_write16(m, cur_bus, dev, func, 0x04, cmd & ~(1<<1) & ~(1<<0)) # disable mem,io

                def show_bar(m, bus, dev, func, bar):
                    bar_addr = 0x10 + bar * 4
                    pci_config_write32(m, cur_bus, dev, func, bar_addr, 0xffffffff)
                    barval = pci_config_read32(m, bus, dev, func, bar_addr)
                    if barval == 0:
                        return 1

                    mask = 0xffff_ffff
                    io = False
                    if barval & 1:
                        io = True
                        mask = 0xffff

                    pci_config_write32(m, bus, dev, func, bar_addr, mask)
                    bar_val = pci_config_read32(m, bus, dev, func, bar_addr)
                    bar_size = ((bar_val & 0xfffffff0) ^ mask) + 1

                    if io:
                        print(f"  IO BAR{bar:02x}: {barval:04x} {bar_size:04x}")
                    else:
                        print(f"  MEMBAR{bar:02x}: {barval:08x} {bar_size:08x}")

                    if (barval & 0x6) == 0x4:
                        size = 2
                    else:
                        size = 1

                    return size

                if headertyp & 1:
                    mem_base0 = pci_config_read32(m, cur_bus, dev, func, 0x20)
                    mem_base = ((mem_base0 & 0xffff) >> 4) << 20
                    mem_limit = ((mem_base0 & 0xffff0000) >> 20) << 20
                    print(f"  MEM: {mem_base:08x} {mem_limit:08x}")

                    io_base0 = pci_config_read16(m, cur_bus, dev, func, 0x1c)
                    io_base = ((io_base0&0xff) >> 4)<<12
                    io_limit = ((io_base0&0xff00) >> 12)<<12
                    print(f"  IO: {io_base:08x} {io_limit:08x}")

                else:
                    bar = 0
                    while bar < 6:
                        size = show_bar(m, cur_bus, dev, func, bar)
                        if size == -1:
                            break
                        bar += size

                pci_config_write32(m, cur_bus, dev, func, 0x30, 0xfffffff0);
                exp_rom = pci_config_read32(m, cur_bus, dev, func, 0x30)
                if exp_rom != 0:
                    print(f"  EXP_ROM: {exp_rom:08x}")
                    show_bar(m, cur_bus, dev, func, 8)
                pci_config_write32(m, cur_bus, dev, func, 0x30, 0xc0001);

                cmd = pci_config_read16(m, cur_bus, dev, func, 0x04)
                if cc == 0x06 and scc == 0x04:
                    bus += 1
                    assigned = bus
                    pci_config_write8(m, cur_bus, dev, func, 0x19, bus) # scbn
                    pci_config_write8(m, cur_bus, dev, func, 0x1a, bus) # sbbn

                    bctrl = pci_config_read16(m, cur_bus, dev, func, 0x3e)
                    if dev == 30:
                        pci_config_write16(m, cur_bus, dev, func, 0x3e, (bctrl | (1<<3) | (1<<2) | (1<<4))) # vga, vga16, isa
                        pci_config_write16(m, cur_bus, dev, func, 0x04, cmd | (1<<1) | (1<<0)) # enable mem,io
                        pci_config_write32(m, cur_bus, dev, func, 0x20, 1<<20)

                    bus = assign_bus(m, bus)
                    pci_config_write8(m, cur_bus, dev, func, 0x1a, bus) # sbbn
                else:
                    pci_config_write16(m, cur_bus, dev, func, 0x04, cmd | (1<<1) | (1<<0)) # enable mem,io


        return bus

    assign_bus(m, 0)

#    if True:
#        bus = 4
#        for dev in range(32):
#            vid = pci_config_read16(m, bus, dev, 0, 0)
#            did = pci_config_read16(m, bus, dev, 0, 2)
#            pi = pci_config_read8(m, bus, dev, 0, 0x09)
#            scc = pci_config_read8(m, bus, dev, 0, 0x0a)
#            cc = pci_config_read8(m, bus, dev, 0, 0x0b)
#            print(f"{dev:02x}: {vid:04x} {did:04x} {pi:02x} {scc:02x} {cc:02x}")
#
#    bctrl = pci_config_read16(m, 0, 30, 0, 0x3e)
#    print(f"BCTRL: {bctrl:x}")
#    pci_config_write16(m, 0, 30, 0, 0x3e, (bctrl | (1<<3) | (1<<2)) & ~(1<<4))
#
#    cmd = pci_config_read16(m, 0, 30, 0, 0x04)
#    print(f"CMD: {cmd:x}")
#    pci_config_write16(m, 0, 30, 0, 0x04, cmd | (1<<1) | (1<<0))
#
#    tbl = [0x86,0x80,0x4e,0x24,0x07,0x00,0x10,0x00,0xe1,0x01,0x04,0x06,0x00,0x00,0x01,0x00,
#        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x04,0x04,0x20,0xf0,0x00,0x80,0x22,
#        0x00,0x98,0xf0,0x98,0x01,0x90,0xf1,0x97,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
#        0x00,0x00,0x00,0x00,0x50,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xff,0x00,0x1a,0x00]
#
#    #for (addr,val) in enumerate(tbl):
#    #    pci_config_write8(m, 0, 30, 0, addr, val)
#
#    pci_config_write8(m, 0, 30, 0, 0x19, 3) # scbn
#    pci_config_write8(m, 0, 30, 0, 0x1a, 3) # sbbn
#    slat = pci_config_read8(m, 0, 30, 0, 0x1b) # sec-latency
#    print(f"SLAT: {slat:x}")


if __name__ == "__main__":
    main()
