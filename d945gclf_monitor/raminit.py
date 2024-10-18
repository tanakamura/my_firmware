import serial
from monitor import *
import monitor

MCHBASE = 0xf0000000


def do_ram_command(m, com):
    write32(m, MCHBASE+0x0200, com | 0x00400)
    v = read32(m, MCHBASE+0x0200)
    print(f"{v:x}")


def main():
    global proc

    to_mon = None
    from_mon = None
    if len(sys.argv) < 2:
        import serial.tools.list_ports
        port = serial.Serial(port='/dev/ttyS0', baudrate=115200 , parity='N', stopbits=1)
        to_mon = port
        from_mon = port
    elif sys.argv[1] == "qemu":
        monitor.proc = subprocess.Popen(["qemu-system-i386", "-serial", "mon:stdio", "-bios", "rom_qemu", "-nographic", "-M", "q35", "-m", "8M"], stdin=subprocess.PIPE, stdout=subprocess.PIPE)
        proc = monitor.proc
        to_mon = proc.stdin
        from_mon = proc.stdout
    else:
        print("Usage: monitor.py [qemu]")
        sys.exit(1)

    m = Machine(to_mon, from_mon)

    pci_config_write16(m, 0, 0x0, 0, 4, 2) # Enable MEM access
    pci_config_write32(m, 0, 0x0, 0, 0x44, MCHBASE|1) # bar = 0x80000000

    def yyyy():
        pmcon2 = pci_config_read8(m, 0, 0x1f, 0, 0xa2)
        print(f"pmcon2 : 0x{pmcon2:02x}")
        if pmcon2 & 4:
            pci_config_write8(m, 0, 0x1f, 0, 0xa2, pmcon2)
            pmcon3 = pci_config_read8(m, 0, 0x1f, 0, 0xa4)
            print(f"pmcon3 : 0x{pmcon3:02x}")
            pmcon3 |= (1<<3)
            pci_config_write8(m, 0, 0x1f, 0, 0xa4, pmcon3)
            outb_ignore_ack(m, 0xcf9, 0xa)
            outb_ignore_ack(m, 0xcf9, 0xe)
            return

        pci_config_write8(m, 0, 0x1f, 0, 0xa2, pmcon2|(1<<7))

        f = open("commands.txt", "r")
        for l in f.readlines():
            vals = l.split(',')

            print(l, end="")

            if vals[1] == "MCH":
                length = vals[0][1]
                addr = int(vals[2], 16)
                expected = int(vals[3], 16)

                if vals[0][0] == "R":
                    if length == 'B':
                        val = read8(m, MCHBASE + addr)
                    elif length == 'H':
                        val = read16(m, MCHBASE + addr)
                    elif length == 'L':
                        val = read32(m, MCHBASE + addr)

                    elif val != expected:
                        print(f"ERROR: addr: 0x{addr:x}, expected: 0x{expected:x}, actual: 0x{val:x}")
                        return
                elif vals[0][0] == "W":
                    if length == 'B':
                        write8(m, MCHBASE + addr, expected)
                    elif length == 'H':
                        write16(m, MCHBASE + addr, expected)
                    elif length == 'L':
                        write32(m, MCHBASE + addr, expected)
            elif vals[1] == "PCI":
                bus = 0
                dev = int(vals[2], 16)
                fn = int(vals[3], 16)
                off = int(vals[4], 16)
                expected = int(vals[5], 16)
                length = vals[0][1]

                if vals[0][0] == "R":
                    if length == 'B':
                        val = pci_config_read8(m, bus, dev, fn, off)
                    elif length == 'H':
                        val = pci_config_read16(m, bus, dev, fn, off)
                    elif length == 'L':
                        val = pci_config_read32(m, bus, dev, fn, off)

                    if False and val != expected:
                        print(f"ERROR: bus: {bus}, dev: {dev}, fn: {fn}, off: 0x{off:x}, expected: 0x{expected:x}, actual: 0x{val:x}")
                        return
                if vals[0][0] == "W":
                    if length == 'B':
                        pci_config_write8(m, bus, dev, fn, off, expected)
                    elif length == 'H':
                        pci_config_write16(m, bus, dev, fn, off, expected)
                    elif length == 'L':
                        pci_config_write32(m, bus, dev, fn, off, expected)

            elif vals[1] == "RAM":
                addr = int(vals[2], 16)
                v = read32(m, addr)
                print(f"RAM: 0x{addr:x} = 0x{v:x}")


    def xxxx():
        pmcon2 = pci_config_read8(m, 0, 0x1f, 0, 0xa2)
        print(f"pmcon2 : 0x{pmcon2:02x}")
        if pmcon2 & 4:
            pci_config_write8(m, 0, 0x1f, 0, 0xa2, pmcon2)
            pmcon3 = pci_config_read8(m, 0, 0x1f, 0, 0xa4)
            print(f"pmcon3 : 0x{pmcon3:02x}")
            pmcon3 |= (1<<3)
            pci_config_write8(m, 0, 0x1f, 0, 0xa4, pmcon3)
            outb_ignore_ack(m, 0xcf9, 0xa)
            outb_ignore_ack(m, 0xcf9, 0xe)
            return

        pci_config_write8(m, 0, 0x1f, 0, 0xa2, pmcon2|(1<<7))

        v0 = read32(m, MCHBASE+0xf14)
        print(f"SLFRCS : {v0:x}")
        write32(m, MCHBASE+0xf14, 3)
        v0 = read32(m, MCHBASE+0xf14)
        print(f"SLFRCS : {v0:x}")

        if True:
            write32(m, MCHBASE+0xc00, 0x20000021)
            v0 = read32(m, MCHBASE+0xc00) # CLKCFG
            print(f"CLKCFG {v0:x}")

            write32(m, MCHBASE+0x0200, 0xf0400)
            v0 = read32(m, MCHBASE+0x0200) # DCC
            print(f"DCC {v0:x}")

            write32(m, MCHBASE+0x100, 0x40404020) # C0DRB rank0-4
            v0 = read32(m, MCHBASE+0x0100)
            print(f"{v0:x}")

            # TOLUD
            pci_config_write8(m, 0, 0, 0, 0x9c, 0x80)
            v0 = pci_config_read8(m, 0, 0, 0, 0x9c)
            print(f"{v0:x}")

            # C0HCTC
            write8(m, MCHBASE+0x37c, 1)
            # WDLLBYPMODE
            write16(m, MCHBASE+0x360, 0x1a5)
            # C0WDLLCMC
            write32(m, MCHBASE+0x36c, 0)
            # C0DRAMW
            write16(m, MCHBASE+0x40c, 0x55)

            # GxSC
            write8(m, MCHBASE+0x410, 0x44)
            write8(m, MCHBASE+0x418, 0x33)
            write8(m, MCHBASE+0x420, 0x00)
            write8(m, MCHBASE+0x428, 0x00)
            write8(m, MCHBASE+0x430, 0x44)
            write8(m, MCHBASE+0x438, 0x44)

            tbl = [0x08070706,
                   0x0a090908,
                   0x0d0c0b0a,
                   0x12100f0e,
                   0x1a181614,
                   0x22201e1c,
                   0x2a282624,
                   0x3934302d,
                   0x0a090908,
                   0x0c0b0b0a,
                   0x0e0d0d0c,
                   0x1211100f,
                   0x19171513,
                   0x211f1d1b,
                   0x2d292623,
                   0x3f393531,
                   0x05050404,
                   0x0b090706,
                   0x13110f0d,
                   0x1d1b1915,
                   0x1f1f1f1f,
                   0x1f1f1f1f,
                   0x1f1f1f1f,
                   0x1f1f1f1f,
                   0x0e0e0d0d,
                   0x100f0f0f,
                   0x1b191310,
                   0x1f1f1f1d,
                   0x1f1f1f1f,
                   0x1f1f1f1f,
                   0x1f1f1f1f,
                   0x1f1f1f1f,
                   0x07070606,
                   0x0e0c0a08,
                   0x17141210,
                   0x201e1c1a,
                   0x28262422,
                   0x302e2c2a,
                   0x38363432,
                   0x3f3e3c3a,
                   0x13131212,
                   0x16151414,
                   0x211d1a18,
                   0x28262422,
                   0x302e2c2a,
                   0x38363432,
                   0x3f3e3c3a,
                   0x3f3f3f3f,
                   0x07070606,
                   0x0e0c0a08,
                   0x17141210,
                   0x201e1c1a,
                   0x28262422,
                   0x302e2c2a,
                   0x38363432,
                   0x3f3e3c3a,
                   0x13131212,
                   0x16151414,
                   0x211d1a18,
                   0x28262422,
                   0x302e2c2a,
                   0x38363432,
                   0x3f3e3c3a,
                   0x3f3f3f3f,
                   0x0c0b0b0b,
                   0x0d0d0c0c,
                   0x100f0e0d,
                   0x15131211,
                   0x1d1b1917,
                   0x2523211f,
                   0x2a282927,
                   0x32302e2c,
                   0x11101010,
                   0x12121111,
                   0x15131312,
                   0x1a181716,
                   0x22201e1c,
                   0x2a282624,
                   0x2f2e2d2c,
                   0x37353331,
                   0x0c0b0b0b,
                   0x0d0d0c0c,
                   0x100f0e0d,
                   0x15131211,
                   0x1d1b1917,
                   0x2523211f,
                   0x2a282927,
                   0x32302e2c,
                   0x09090808,
                   0x0c0b0b0a,
                   0x100f0e0d,
                   0x14131211,
                   0x18171615,
                   0x1e1c1a19,
                   0x26242220,
                   0x2e2c2a28,
                   0x08070706,
                   0x0a090908,
                   0x0d0c0b0a,
                   0x12100f0e,
                   0x1a181614,
                   0x22201e1c,
                   0x2a282624,
                   0x3934302d,
                   0x0a090908,
                   0x0c0b0b0a,
                   0x0e0d0d0c,
                   0x1211100f,
                   0x19171513,
                   0x211f1d1b,
                   0x2d292623,
                   0x3f393531,
                   0x13121110,
                   0x17161514,
                   0x1b1a1918,
                   0x1f1e1d1c,
                   0x23222120,
                   0x27262524,
                   0x2b2a2928,
                   0x2f2e2d2c,
                   0x13121110,
                   0x17161514,
                   0x1b1a1918,
                   0x1f1e1d1c,
                   0x23222120,
                   0x27262524,
                   0x2b2a2928,
                   0x2f2e2d2c]

            # GxSRPUT, GxSRPDT
            for (val,off) in zip(tbl, range(0x500, 0x700, 4)):
                # print(f"0x{off:x}: 0x{val:08x}")
                write32(m, MCHBASE+off, val)

            # GBRCOMPCTL
            v0 = read32(m, MCHBASE+0x400)
            print(f"GBRCOMPCTL {v0:x}")
            v0 = v0 & (~((1 << 29) | (1 << 26) | (3 << 21) | (3 << 2)))
            v0 = v0 | (3<<27) | (3<<0)
            write32(m, MCHBASE+0x0400, v0)
            write32(m, MCHBASE+0x0400, 1<<10)

            # DQSMT
            write16(m, MCHBASE + 0x2f4, (1<<13) | (0xc))
            v0 = read16(m, MCHBASE + 0x2f4)
            print(f"DQSMT {v0:x}")

            # DLL (533MHz)
            for i in range(4):
                write32(m, MCHBASE+0x300 + i*0x10 + 0, 0x24242424)
                write32(m, MCHBASE+0x300 + i*0x10 + 4, 0x24242424)

                write32(m, MCHBASE+0x380 + i*0x10 + 0, 0x24242424)
                write32(m, MCHBASE+0x380 + i*0x10 + 4, 0x24242424)

            # ODTC
            write32(m, MCHBASE+0x284, 0x10004040)
            # SMSRCTL?
            v0 = read32(m, MCHBASE+0x408)
            print(f"SMSRCTL {v0:x}")
            v0 |= 1
            write32(m, MCHBASE+0x408, v0)

            v0 = read32(m, MCHBASE+0x400)
            print(f"GBRCOMPCTL {v0:x}")
            v0 = v0 | (1<<8)
            write32(m, MCHBASE+0x0400, v0)

            # RCVENMT
            write32(m, MCHBASE+0x2f8, 0x50acf)
            # DRTST
            write32(m, MCHBASE+0x2a8, 0x03ff00fc)

            # C0DRC1
            write32(m, MCHBASE+0x124, 0x80000502)
            # C1DRC1
            write32(m, MCHBASE+0x1a4, 0x00000402)

            # C0DCLKDIS
            write32(m, MCHBASE+0x10c, 0x00050007)
            # C1DCLKDIS
            write32(m, MCHBASE+0x18c, 0x00000000)

        do_ram_command(m, 0x1<<16) # nop
        v0 = read8(m, 0)
        print(f"normal {v0:x}")
        do_ram_command(m, 0x2<<16) # all bank precharge
        v0 = read8(m, 0)
        print(f"all bank precharge {v0:x}")
        do_ram_command(m, 0x4<<16) # emrs
        v0 = read8(m, 0)
        print(f"emrs {v0:x}")
#
#    do_ram_command(m, 0x2<<16) # prechage
#    v0 = read8(m, 0)
#    print(f"precharge {v0:x}")
#
#    do_ram_command(m, (0x6<<16) | (0x1<<21) ) #  emrs2
#    v0 = read8(m, 0)
#    print(f"emrs2 {v0:x}")

    yyyy()


if __name__ == "__main__":
    main()
