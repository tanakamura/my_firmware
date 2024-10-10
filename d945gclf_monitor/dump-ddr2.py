def main():
    f = open("spd.bin", "rb")
    buf = f.read()

    print(f"Clock cycle time at highest CAS latency : {buf[9]>>4}.{buf[9]&0xf}")
    v = ((buf[10]>>4)*10) + (buf[10]&0xf)
    print(f"SDRAM access time from clock            : {v/100}")
    print(f"Primary SDRAM Width                     : {buf[13]:x}")
    print(f"burst length supported                  : {buf[16]:x}")
    print(f"banks per SDRAM device                  : {buf[17]:x}")
    print(f"CAS supported                           : {buf[18]:08b}")

    print(f"Clock cycle time at medium CAS latency  : {buf[23]>>4}.{buf[23]&0xf}")
    v = ((buf[24]>>4)*10) + (buf[24]&0xf)
    print(f"Data access time from clock             : {v/100}")

    print(f"Clock cycle time at short  CAS latency  : {buf[25]>>4}.{buf[25]&0xf}")
    v = ((buf[26]>>4)*10) + (buf[26]&0xf)
    print(f"Data access time from clock             : {v/100}")

    v = ((buf[27]>>2)) + ((buf[27]&0x3)/4.0)
    print(f"tRP                                     : {v}")
    v = ((buf[28]>>2)) + ((buf[28]&0x3)/4.0)
    print(f"tRRD                                    : {v}")
    v = ((buf[29]>>2)) + ((buf[29]&0x3)/4.0)
    print(f"tRCD                                    : {v}")
    print(f"tRAS                                    : {buf[30]}")
    print(f"size of each rank                       : {buf[31]:08b}")

    pass


if __name__ == "__main__":
    main()
