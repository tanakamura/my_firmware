import sys
import subprocess
import atexit
import readline
import struct
import serial

from monitor import *

def main():
    to_mon = None
    from_mon = None
    if len(sys.argv) < 2:
        import serial.tools.list_ports
        port = serial.Serial(port='/dev/ttyS0', baudrate=115200 , parity='N', stopbits=1)
        to_mon = port
        from_mon = port
    elif sys.argv[1] == "qemu":
        proc = subprocess.Popen(["qemu-system-i386", "-serial", "mon:stdio", "-bios", "rom_qemu", "-nographic", "-M", "q35", "-m", "8M"], stdin=subprocess.PIPE, stdout=subprocess.PIPE)
        to_mon = proc.stdin
        from_mon = proc.stdout
    else:
        print("Usage: monitor.py [qemu]")
        sys.eixt(1)

    m = Machine(to_mon, from_mon)

    atexit.register(cleanup)

    init(m)
    SMBASE = 0x1000
    HST_STS = 0x00
    HST_CNT = 0x02
    HST_CMD = 0x03
    XMIT_SLVA = 0x04
    HST_D0 = 0x05
    HST_D1 = 0x06

    HST_STS_INTR = 1<<1
    # RCV_SLVA = 0x44

    pci_config_write16(m, 0, 0x1f, 3, 4, 1) # Enable IO access
    pci_config_write32(m, 0, 0x1f, 3, 0x20, SMBASE) # bar = 0x80000000
    pci_config_write8(m, 0, 0x1f, 3, 0x40, 1) # enable smbus

    def read_byte(m, addr):
        # reset

        outb(m, SMBASE+HST_CNT, 0x0)
        outb(m, SMBASE+HST_STS, 0xff)

        outb(m, SMBASE+HST_CNT, 2<<2)          # read
        outb(m, SMBASE+XMIT_SLVA, (0x50<<1)|1) # read from 0x50
        outb(m, SMBASE+HST_CMD, addr)           # read addr
        outb(m, SMBASE+HST_D0, 0)              # clear
        outb(m, SMBASE+HST_D1, 0)              # clear
        outb(m, SMBASE+HST_CNT, (2<<2)|(1<<6))

        done = False
        for i in range(128):
            d = inb(m, SMBASE+HST_STS)
            if d & HST_STS_INTR:
                done = True
                break

        if not done:
            print(hex(d))
            raise(Exception("unko"))

        return inb(m, SMBASE+HST_D0)

    spd_len = read_byte(m, 0)
    spd = [0]*spd_len

    for i in range(spd_len):
        x = read_byte(m, i)
        print(f"{i:02x} {x:02x}")
        spd[i] = x

    def dump_hst(m):
        tbl = [("STS", HST_STS),
               ("CNT", HST_CNT),
               ("CMD", HST_CMD),
               ("XMIT_SLVA", XMIT_SLVA),
               ("D0", HST_D0),
            ("D1", HST_D1)]

        for (name,idx) in tbl:
            print(f"{name:10s} {inb(m, SMBASE+idx):08x}")

    f = open("spd.bin", "wb")
    f.write(bytes(spd))
    f.close()
    # dump_hst(m)


if __name__ == "__main__":
    main()
