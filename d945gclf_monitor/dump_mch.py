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
    import serial.tools.list_ports
    port = serial.Serial(port='/dev/ttyS0', baudrate=115200 , parity='N', stopbits=1)
    to_mon = port
    from_mon = port

    m = Machine(to_mon, from_mon)

    init(m)

    pciexbar = (pci_config_read32(m, 0, 0, 0, 0x48)) & 0xfffffff0
    print(f"PCIEXBAR {pciexbar:x}")

    write32(m, 0xf0000044, 0xfed10001)

    bar = read32(m, pciexbar + 0x44) & 0xfffffff0
    print(f"mchbar {bar:x}")

    for i in range(0, 0x1000, 4):
        v = read32(m, bar + i)
        if v != 0:
            print(f"0x{i:04x}: 0x{v:08x}")


if __name__ == "__main__":
    main()
