import sys
import subprocess
import atexit
import readline
import struct
import serial
import argparse
import socket
import atexit

global proc
proc = None
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

LOADBIN = 16
RUNBIN = 17


class Machine:
    def __init__(self, to_mon, from_mon):
        self.to_mon = to_mon
        self.from_mon = from_mon

def open_machine():
    args = argparse.ArgumentParser()
    args.add_argument("--dev_type", choices=["serial", "uds"], required=True, type=str)
    args.add_argument("--dev_path", required=True, type=str)
    args.add_argument("--binary", type=str)

    args = args.parse_args()
    if args.dev_type == "serial":
        port = serial.Serial(port=args.dev_path, baudrate=115200 , parity='N', stopbits=1)
        to_mon = port
        from_mon = port
    elif args.dev_type == "uds":
        s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        s.connect(args.dev_path)
        to_mon = s.makefile("wb")
        from_mon = s.makefile("rb")

    atexit.register(cleanup)

    return (Machine(to_mon, from_mon), args)

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

def read16(m:Machine, addr:int):
    data = struct.pack("<BI", READ16, addr)
    m.to_mon.write(data)
    m.to_mon.flush()
    data = m.from_mon.read(2)
    return struct.unpack("<H", data)[0]

def read32(m:Machine, addr:int):
    data = struct.pack("<BI", READ32, addr)
    m.to_mon.write(data)
    m.to_mon.flush()
    data = m.from_mon.read(4)
    return struct.unpack("<I", data)[0]

def write8(m:Machine, addr:int, val:int):
    data = struct.pack("<BIB", WRITE8, addr, val)
    m.to_mon.write(data)
    m.to_mon.flush()
    ack = m.from_mon.read(1)
    if ack[0] != 0xfe:
        print(f"write8 failed {ack[0]:x}")
        sys.exit(1)

def write16(m:Machine, addr:int, val:int):
    data = struct.pack("<BIH", WRITE16, addr, val)
    m.to_mon.write(data)
    m.to_mon.flush()
    ack = m.from_mon.read(1)
    if ack[0] != 0xfe:
        print(f"write16 failed {ack[0]:x}")
        sys.exit(1)

def write32(m:Machine, addr:int, val:int):
    data = struct.pack("<BII", WRITE32, addr, val)
    m.to_mon.write(data)
    m.to_mon.flush()
    ack = m.from_mon.read(1)
    if ack[0] != 0xfe:
        print(f"write32 failed {ack[0]:x}")
        sys.exit(1)

def outb(m:Machine, port:int, data:int):
    data = struct.pack("<BHB", OUT8, port, data)
    m.to_mon.write(data)
    m.to_mon.flush()
    ack = m.from_mon.read(1)
    if ack[0] != 0xfe:
        print("Outb failed {ack:x}")
        sys.exit(1)

def outb_ignore_ack(m:Machine, port:int, data:int):
    data = struct.pack("<BHB", OUT8, port, data)
    m.to_mon.write(data)
    m.to_mon.flush()

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

def loadbin(m:Machine, binary):
    sumbyte = 0
    for i in binary:
        sumbyte ^= i

    data = struct.pack("<BIB", LOADBIN, len(binary), sumbyte)

    m.to_mon.write(data)
    m.to_mon.flush()

    m.to_mon.write(binary)
    m.to_mon.flush()

    ack = m.from_mon.read(1)
    if ack[0] != 0xfe:
        print("loadbin failed")
        sys.exit(1)

def runbin(m:Machine):
    data = struct.pack("<B", RUNBIN)
    m.to_mon.write(data)
    m.to_mon.flush()

    while True:
        c = m.from_mon.read(1)
        if c == b'\xff':
            break
        sys.stdout.buffer.write(c)
        sys.stdout.flush()

    data = m.from_mon.read(4)
    x = struct.unpack("<I", data)[0]
    print(f"runbin: {x:x}")
    return x

def init(m:Machine):
    data = struct.pack("<B", INIT)
    m.to_mon.write(data)
    m.to_mon.flush()
    data = m.from_mon.read(1)
    print(data[0])
    if data[0] != 1:
        print("Init failed")
        sys.exit(1)

atexit.register(cleanup)

