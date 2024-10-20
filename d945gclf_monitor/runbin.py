import serial
from monitor import *
import monitor
import sys

MCHBASE = 0xf0000000


def do_ram_command(m, com):
    write32(m, MCHBASE+0x0200, com | 0x00400)
    v = read32(m, MCHBASE+0x0200)
    print(f"{v:x}")


def main():
    global proc

    if sys.argv[1] == "native":
        import serial.tools.list_ports
        port = serial.Serial(port='/dev/ttyS0', baudrate=115200 , parity='N', stopbits=1)
        to_mon = port
        from_mon = port
    elif sys.argv[1] == "qemu":
        monitor.proc = subprocess.Popen(["qemu-system-i386", "-s", "-serial", "mon:stdio", "-bios", "rom_qemu", "-nographic", "-M", "q35", "-m", "1G"], stdin=subprocess.PIPE, stdout=subprocess.PIPE)
        proc = monitor.proc
        to_mon = proc.stdin
        from_mon = proc.stdout
    else:
        print("Usage: monitor.py [qemu]")
        sys.exit(1)

    m = Machine(to_mon, from_mon)

    init(m)

    #outb(m, 0x3fa, 0x3fa)
    outb(m, 0x3fa, 0x7)
    fifo = inb(m, 0x3fa)
    print(f"fifo = {fifo:x}")

    binary = open(sys.argv[2], "rb").read()

    loadbin(m, binary)
    print("loaded")
    r = runbin(m)
    print(f"{r:x}")


if __name__ == "__main__":
    main()
