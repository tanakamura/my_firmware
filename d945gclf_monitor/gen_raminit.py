import sys
import struct

MCH_BASE = 0xfed10000
PCI_CONFIG_BASE = 0xf0000000

def main():
    outf = sys.argv[1]
    inf = sys.argv[2]
    mode = sys.argv[3]

    outf = open(outf, "w")
    inf = open(inf, "r")

    if mode == "asm":
        def read8(addr):
            print(f"movb 0x{addr:08x}, %al", file=outf)
        def read16(addr):
            print(f"movw 0x{addr:08x}, %ax", file=outf)
        def read32(addr):
            print(f"movl 0x{addr:08x}, %eax", file=outf)


        def write8(addr, val):
            print(f"movb $0x{val:02x}, %al", file=outf)
            print(f"movb %al, 0x{addr:08x}", file=outf)
        def write16(addr, val):
            print(f"movw $0x{val:04x}, %ax", file=outf)
            print(f"movw %ax, 0x{addr:08x}", file=outf)
        def write32(addr, val):
            print(f"movl $0x{val:08x}, %eax", file=outf)
            print(f"movl %eax, 0x{addr:08x}", file=outf)
    else:
        def read8(addr):
            print(f"core::ptr::read_volatile(0x{addr:08x} as *const u8);", file=outf)
        def read16(addr):
            print(f"core::ptr::read_volatile(0x{addr:08x} as *const u16);", file=outf)
        def read32(addr):
            print(f"core::ptr::read_volatile(0x{addr:08x} as *const u32);", file=outf)

        def write8(addr, val):
            print(f"core::ptr::write_volatile(0x{addr:08x} as *mut u8, 0x{val:08x});", file=outf)
        def write16(addr, val):
            print(f"core::ptr::write_volatile(0x{addr:08x} as *mut u16, 0x{val:08x});", file=outf)
        def write32(addr, val):
            print(f"core::ptr::write_volatile(0x{addr:08x} as *mut u32, 0x{val:08x});", file=outf)


    print(f"// set MCH_BASE to 0x{MCH_BASE:08x}", file=outf)
    addr = PCI_CONFIG_BASE + (0 << 20) + (0 << 15) + (0 << 12) + 0x44
    write32(addr, MCH_BASE | 1)

    for (line,l) in enumerate(inf.readlines()):
        if line % 100 == 0:
            l100 = line / 100
            l100_16 = l100 % 16
            c = "0123456789abcdef"[int(l100_16)]
            # print(f"uart_put8(b'{c}');", file=outf)

        vals = l.split(',')
        print("//" + l, end="", file=outf)
        if vals[1] == "MCH":
            length = vals[0][1]
            addr = int(vals[2], 16)
            expected = int(vals[3], 16)

            if vals[0][0] == "R":
                if length == 'B':
                    read8(MCH_BASE + addr)
                elif length == 'H':
                    read16(MCH_BASE + addr)
                elif length == 'L':
                    read32(MCH_BASE + addr)
            elif vals[0][0] == "W":
                if length == 'B':
                    write8(MCH_BASE + addr, expected)
                elif length == 'H':
                    write16(MCH_BASE + addr, expected)
                elif length == 'L':
                    write32(MCH_BASE + addr, expected)
        elif vals[1] == "PCI":
            bus = 0
            dev = int(vals[2], 16)
            fn = int(vals[3], 16)
            off = int(vals[4], 16)
            expected = int(vals[5], 16)
            length = vals[0][1]

            addr = PCI_CONFIG_BASE + (bus << 20) + (dev << 15) + (fn << 12) + off

            if vals[0][0] == "R":
                if length == 'B':
                    read8(addr)
                elif length == 'H':
                    read16(addr)
                elif length == 'L':
                    read32(addr)

            if vals[0][0] == "W":
                if length == 'B':
                    write8(addr, expected)
                elif length == 'H':
                    write16(addr, expected)
                elif length == 'L':
                    write32(addr, expected)

        elif vals[1] == "RAM":
            addr = int(vals[2], 16)
            read32(addr)



if __name__ == "__main__":
    main()
