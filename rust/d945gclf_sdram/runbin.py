import serial
import sys
import struct


def main():
    global proc

    port = serial.Serial(port='/dev/ttyS0', baudrate=115200 , parity='N', stopbits=1)
    bytes = open(sys.argv[1], "rb").read()

    len_bytes = struct.pack("<I", len(bytes))
    port.write(len_bytes)

    sum = 0
    for b in bytes:
        sum = sum ^ b

    sum_bytes = struct.pack("<B", sum)
    port.write(sum_bytes)
    port.write(bytes)


if __name__ == "__main__":
    main()
