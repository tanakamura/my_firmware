import sys
import subprocess
import atexit
import readline
import struct
import serial

INIT = 127

READ8 = 2
READ16 = 3
READ32 = 4

WRITE8 = 5
WRITE16 = 6
WRITE32 = 7

IN8 = 8
IN16 = 9
IN32 = 10

OUT8 = 11
OUT16 = 12
OUT32 = 13

RDMSR = 14
WRMSR = 15



class Machine:
    def __init__(self, to_mon, from_mon):
        self.to_mon = to_mon
        self.from_mon = from_mon

def cleanup():
    global proc
    if proc:
        proc.kill()


def read8(m:Machine, addr:int):
    data = struct.pack("<BI", READ8, addr)
    m.to_mon.write(data)
    m.to_mon.flush()
    data = m.from_mon.read(1)
    return data[0]

def read32(m:Machine, addr:int):
    data = struct.pack("<BI", READ32, addr)
    m.to_mon.write(data)
    m.to_mon.flush()
    data = m.from_mon.read(4)
    return struct.unpack("<I", data)[0]

def outb(m:Machine, port:int, data:int):
    data = struct.pack("<BHB", OUT8, port, data)
    m.to_mon.write(data)
    m.to_mon.flush()
    ack = m.from_mon.read(1)
    if ack[0] != 0xfe:
        print("Outb failed")
        sys.exit(1)
def outw(m:Machine, port:int, data:int):
    data = struct.pack("<BHH", OUT16, port, data)
    m.to_mon.write(data)
    m.to_mon.flush()
    ack = m.from_mon.read(1)
    if ack[0] != 0xfe:
        print("Outb failed")
        sys.exit(1)

def outl(m:Machine, port:int, data:int):
    data = struct.pack("<BHI", OUT32, port, data)
    m.to_mon.write(data)
    m.to_mon.flush()
    ack = m.from_mon.read(1)
    if ack[0] != 0xfe:
        print("Outb failed")
        sys.exit(1)

def inb(m:Machine, port:int):
    data = struct.pack("<BH", IN8, port)
    m.to_mon.write(data)
    m.to_mon.flush()
    data = m.from_mon.read(1)
    return data[0]
def inw(m:Machine, port:int):
    data = struct.pack("<BH", IN16, port)
    m.to_mon.write(data)
    m.to_mon.flush()
    data = m.from_mon.read(2)
    return struct.unpack("<H", data)[0]
def inl(m:Machine, port:int):
    data = struct.pack("<BH", IN32, port)
    m.to_mon.write(data)
    m.to_mon.flush()
    data = m.from_mon.read(4)
    return struct.unpack("<I", data)[0]

def pci_config_read8(m:Machine, bus:int, dev:int, func:int, offset:int):
    outl(m, 0xcf8, (1 << 31) | (bus << 16) | (dev << 11) | (func << 8) | (offset&0xfc))
    return inb(m, 0xcfc+(offset&3))
def pci_config_read16(m:Machine, bus:int, dev:int, func:int, offset:int):
    outl(m, 0xcf8, (1 << 31) | (bus << 16) | (dev << 11) | (func << 8) | (offset&0xfc))
    return inw(m, 0xcfc+(offset&3))
def pci_config_read32(m:Machine, bus:int, dev:int, func:int, offset:int):
    outl(m, 0xcf8, (1 << 31) | (bus << 16) | (dev << 11) | (func << 8) | (offset&0xfc))
    return inl(m, 0xcfc+(offset&3))

def pci_config_write8(m:Machine, bus:int, dev:int, func:int, offset:int, data:int):
    outl(m, 0xcf8, (1 << 31) | (bus << 16) | (dev << 11) | (func << 8) | (offset&0xfc))
    return outb(m, 0xcfc+(offset&3), data)
def pci_config_write16(m:Machine, bus:int, dev:int, func:int, offset:int, data:int):
    outl(m, 0xcf8, (1 << 31) | (bus << 16) | (dev << 11) | (func << 8) | (offset&0xfc))
    return outw(m, 0xcfc+(offset&3), data)
def pci_config_write32(m:Machine, bus:int, dev:int, func:int, offset:int, data:int):
    outl(m, 0xcf8, (1 << 31) | (bus << 16) | (dev << 11) | (func << 8) | (offset&0xfc))
    return outl(m, 0xcfc+(offset&3), data)

def init(m:Machine):
    data = struct.pack("<B", INIT)
    m.to_mon.write(data)
    m.to_mon.flush()
    data = m.from_mon.read(1)
    if data[0] != 1:
        print("Init failed")
        sys.exit(1)

atexit.register(cleanup)

def main():
    to_mon = None
    from_mon = None
    global proc
    proc = None
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
