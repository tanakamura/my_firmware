import serial
from monitor import *
import monitor

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

    #pci_config_write16(m, 0, 0x0, 0, 4, 2) # Enable MEM access
    #pci_config_write32(m, 0, 0x0, 0, 0x44, MCHBASE|1) # bar = 0x80000000
    #
    ## TOLUD
    #pci_config_write8(m, 0, 0, 0, 0x9c, 0x4)
    #v0 = pci_config_read8(m, 0, 0, 0, 0x9c)

    epbar = 0x7fffffff
    mchbar = 0x7fffffff
    rcba = 0x7fffffff
    actual_io = False
    prev_dcc = False
    dcc = False

    filtered = open("commands.txt", "w")

    def in_mchbar(a):
        if (a&0xfffff000) == mchbar:
            return True

    def pci_io(a):
        if (addr&0xFF000000) == 0xf0000000:
            fn = (addr >> 12) & 0x7
            dev = (addr >> 15) & 0x1f
            off = addr & 0xFff

            return (dev,fn,off)
        return None



    d31f0_tbl = {0xA0:"PMCON1",0xA2:"PMCON2",0xA4:"PMCON3"}
    d0f0_tbl = {0x54:"DEVEN",0x9c:"TOLUD",0x200:"DCC"}

    def filter_mmio(w):
        nonlocal epbar
        nonlocal mchbar
        nonlocal annotate
        nonlocal rcba

        if (addr&0xFF000000) == 0xf0000000:
            fn = (addr >> 12) & 0x7
            dev = (addr >> 15) & 0x1f
            off = addr & 0xFff

            annotate = f" #  dev:{dev} fn:{fn} off:{off:x}"

            if dev == 31 and fn == 0 :
                if off in d31f0_tbl:
                    annotate = f" #  dev:{dev} fn:{fn} off:{off:x} {d31f0_tbl[off]}"
            if dev == 0 and fn == 0 :
                if off in d0f0_tbl:
                    annotate = f" #  dev:{dev} fn:{fn} off:{off:x} {d0f0_tbl[off]}"

            if w and dev == 0 and fn == 0:
                if off == 0x40:
                    epbar = val & 0xfffffff0
                elif off == 0x44:
                    mchbar = val & 0xfffffff0
            if w and dev == 31 and fn == 0:
                if off == 0xf0:
                    rcba = val & 0xfffffff0

        if (addr&0xfffff000) == epbar:
            off = addr & 0xfff
            annotate = f" # EPBAR off={off:x}"
        elif (addr&0xfffff000) == mchbar:
            off = addr & 0xfff
            annotate = f" # MCHBAR off={off:x}"
        elif (addr&0xfffff000) == rcba:
            off = addr & 0xfff
            annotate = f" # RCBA off={off:x}"

    f = open("record.txt", "r")
    for l in f.readlines():
        vals = l.split(',')
        annotate = ""
        if vals[0][0] == "R":
            length = vals[0][1]
            addr = int(vals[1], 16)
            expected = int(vals[2], 16)

            ignore = False

            filter_mmio(False)

            if in_mchbar(addr):
                print(f'R{length},MCH,{addr-mchbar:x},{expected:x}', file=filtered)

            p = pci_io(addr);
            if p:
                dev = p[0]
                fn = p[1]
                off = p[2]
                if dev == 31 and fn == 0 and off in d31f0_tbl:
                    print(f'R{length},PCI,{dev:x},{fn:x},{off:x},{val:x}', file=filtered)

            if prev_dcc and addr < 0x20000000:
                print(f'R{length},RAM,{addr:x}',file=filtered)


            if actual_io:
                if ignore:
                    continue
                else:
                    if length == 'B':
                        val = read8(m, addr)
                    elif length == 'H':
                        val = read16(m, addr)
                    elif length == 'L':
                        val = read32(m, addr)

                    elif val != expected:
                        print(f"ERROR: addr: 0x{addr:x}, expected: 0x{expected:x}, actual: 0x{val:x}")
                        return
        elif vals[0][0] == "I":
            length = vals[0][1]
            addr = int(vals[1], 16)
            expected = int(vals[2], 16)

            ignore = False

            if actual_io:
                if ignore:
                    continue
                else:
                    if length == 'B':
                        val = inb(m, addr)
                    elif length == 'H':
                        val = inw(m, addr)
                    elif length == 'L':
                        val = inl(m, addr)

                    elif val != expected:
                        print(f"ERROR: addr: 0x{addr:x}, expected: 0x{expected:x}, actual: 0x{val:x}")
                        return

        elif vals[0][0] == "W":
            length = vals[0][1]
            addr = int(vals[1], 16)
            val = int(vals[2], 16)

            filter_mmio(True)

            if in_mchbar(addr):
                print(f'W{length},MCH,{addr-mchbar:x},{val:x}', file=filtered)
                if (addr & 0xfff) == 0x200:
                    dcc = True

            p = pci_io(addr);
            if p:
                dev = p[0]
                fn = p[1]
                off = p[2]
                if dev == 31 and fn == 0 and off in d31f0_tbl:
                    print(f'W{length},PCI,{dev:x},{fn:x},{off:x},{val:x}', file=filtered)
                if dev == 0 and fn == 0 and off in d0f0_tbl:
                    print(f'W{length},PCI,{dev:x},{fn:x},{off:x},{val:x}', file=filtered)
            if actual_io:
                if length == 'B':
                    write8(m, addr, val)
                elif length == 'H':
                    write16(m, addr, val)
                elif length == 'L':
                    write32(m,  addr, val)

        elif vals[0][0] == "O":
            length = vals[0][1]
            addr = int(vals[1], 16)
            val = int(vals[2], 16)

            if actual_io:
                if length == 'B':
                    outb(m, addr, val)
                elif length == 'H':
                    outw(m, addr, val)
                elif length == 'L':
                    outl(m,  addr, val)

        else:
            print("xx")
            raise Exeption("XX")

        print(l.rstrip('\n')+annotate,end="\n")
        prev_dcc = dcc
        dcc = False


if __name__ == "__main__":
    main()
