from monitor import *

def align_up(v, a):
    return (v + a - 1) & ~(a - 1)

class BusAssigner:
    def __init__(self, bus, devices, mem_start, io_start):
        self.bus = bus
        self.devices = devices
        self.mem_next = mem_start
        self.io_next = io_start
        self.cur_bus = 0

        self.config_read8 = pci_config_read8
        self.config_read16 = pci_config_read16
        self.config_read32 = pci_config_read32

        self.config_write8 = pci_config_write8
        self.config_write16 = pci_config_write16
        self.config_write32 = pci_config_write32

    def assign(m):
        cur_bus = self.bus
        for dev in range(32):
            vid = self.config_read16(m, cur_bus, dev, 0, 0)
            if vid == 0xffff:
                continue
            headertyp = self.config_read8(m, cur_bus, dev, 0, 0x0e)

            if headertyp & 0x80:
                # multi function
                num_func = 8
            else:
                num_func = 1

            for func in range(num_func):
                vid = self.config_read16(m, cur_bus, dev, func, 0)
                if vid == 0xffff:
                    continue

                did = self.config_read16(m, cur_bus, dev, func, 2)
                pi = self.config_read8(m, cur_bus, dev, func, 0x09)
                scc = self.config_read8(m, cur_bus, dev, func, 0x0a)
                cc = self.config_read8(m, cur_bus, dev, func, 0x0b)

                print(f"{cur_bus:02x}:{dev:02x}:{func:02x} {vid:04x} {did:04x} {pi:02x} {scc:02x} {cc:02x}")

                cmd = self.config_read16(m, cur_bus, dev, func, 0x04)
                self.config_write16(m, cur_bus, dev, func, 0x04, cmd & ~(1<<1) & ~(1<<0)) # disable mem,io

                is_bridge = False

                if headertyp & 1: # bridge
                    is_bridge = True
                    mem_base0 = self.config_read32(m, cur_bus, dev, func, 0x20)
                    bridge_mem_start = align_up(self.mem_next, 1024*1024)
                    self.mem_next = bridge_mem_start

                    bridge_io_start = align_up(self.io_next, 4096)
                    self.io_next = bridge_io_start

                    mem_base = ((mem_base0 & 0xffff) >> 4) << 20
                    mem_limit = ((mem_base0 & 0xffff0000) >> 20) << 20
                    print(f"  MEM: {mem_base:08x} {mem_limit:08x}")

                    io_base0 = self.config_read16(m, cur_bus, dev, func, 0x1c)
                    io_base = ((io_base0&0xff) >> 4)<<12
                    io_limit = ((io_base0&0xff00) >> 12)<<12
                    print(f"  IO: {io_base:08x} {io_limit:08x}")

                else:
                    for bar in range(6):
                        self.config_write(m, cur_bus, dev, func, 0x10 + bar * 4, 0xffffffff)
                        barval = self.config_read32(m, cur_bus, dev, func, 0x10 + bar * 4)
                        if barval & 1: # io
                            bar = barval & 0xfffffffc
                        else:
                            bar = barval & 0xfffffff0
                        if show_bar(m, cur_bus, dev, func, bar) == False:
                            break

                exp_rom = self.config_read32(m, cur_bus, dev, func, 0x30)
                if exp_rom != 0:
                    print(f"  EXP_ROM: {exp_rom:08x}")

                cmd = self.config_read16(m, cur_bus, dev, func, 0x04)
                if cc == 0x06 and scc == 0x04:
                    bus += 1
                    assigned = bus
                    self.config_write8(m, cur_bus, dev, func, 0x19, bus) # scbn
                    self.config_write8(m, cur_bus, dev, func, 0x1a, bus) # sbbn

                    bctrl = self.config_read16(m, cur_bus, dev, func, 0x3e)
                    if dev == 30:
                        self.config_write16(m, cur_bus, dev, func, 0x3e, (bctrl | (1<<3) | (1<<2) | (1<<4))) # vga, vga16, isa
                        self.config_write16(m, cur_bus, dev, func, 0x04, cmd | (1<<1) | (1<<0)) # enable mem,io
                        self.config_write32(m, cur_bus, dev, func, 0x20, 1<<20)

                    bus = assign_bus(m, bus)
                    self.config_write8(m, cur_bus, dev, func, 0x1a, bus) # sbbn
                else:
                    self.config_write16(m, cur_bus, dev, func, 0x04, cmd | (1<<1) | (1<<0)) # enable mem,io

                if is_bridge:
                    bridge_io_start_4k = bridge_io_start/4096
                    bridge_io_end = align_up(self.io_next, 4096)
                    self.io_next = bridge_io_end

                    bridge_io_end_4k = (bridge_io_end-1)/4096

                    io_base = (bridge_io_end_4k<<12) | (bridge_io_start_4k<<4)
                    self.config_write16(m, cur_bus, dev, func, 0x1c, io_base)


                    bridge_mem_start_1m = bridge_mem_start/(1024*1024)
                    bridge_mem_end = align_up(self.mem_next, 1024*1024)
                    self.mem_next = bridge_mem_end

                    bridge_mem_end_1m = (bridge_mem_end-1)/(1024*1024)

                    mem_base = (bridge_mem_end_1m<<20) | (bridge_mem_start_1m<<4)
                    self.config_write32(m, cur_bus, dev, func, 0x20, mem_base)

                self.config_write16(m, cur_bus, dev, func, 0x04, cmd | (1<<1) | (1<<0)) # enable mem,io
