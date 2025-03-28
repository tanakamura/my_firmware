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

    oprom = read32(m, 0xc0000)
    print(f"OPROM: {oprom:x}")

    v3cc = inb(m, 0x3cc)
    print(f"V3CC: {v3cc:x}")


if __name__ == "__main__":
    main()
