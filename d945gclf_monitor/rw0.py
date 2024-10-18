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

    write32(m, 0, 2)
    v = read32(m, 0)
    print(v)


if __name__ == "__main__":
    main()
