use crate::{println, uart};
use alloc::vec::Vec;
use x86::io::{inb, inl, inw, outb, outl, outw};

fn align_up(v: u64, a: u64) -> u64 {
    (v + a - 1) & !(a - 1)
}

pub trait PciConfigIf {
    fn read8(&self, adr: u32, offset: u32) -> u8;
    fn read16(&self, adr: u32, offset: u32) -> u16;
    fn read32(&self, adr: u32, offset: u32) -> u32;
    fn write8(&self, adr: u32, offset: u32, data: u8);
    fn write16(&self, adr: u32, offset: u32, data: u16);
    fn write32(&self, adr: u32, offset: u32, data: u32);
    fn bus_dev_fn_to_adr(&self, bus: u8, dev: u8, fnc: u8) -> u32;
}

pub struct IOPciConfig {}

impl PciConfigIf for IOPciConfig {
    fn read8(&self, adr: u32, offset: u32) -> u8 {
        unsafe {
            outl(0xcf8, (1 << 31) | adr | (offset & 0xfc));
            inb((0xcfc + (offset & 3)) as u16)
        }
    }
    fn read16(&self, adr: u32, offset: u32) -> u16 {
        unsafe {
            outl(0xcf8, (1 << 31) | adr | (offset & 0xfc));
            inw((0xcfc + (offset & 3)) as u16)
        }
    }
    fn read32(&self, adr: u32, offset: u32) -> u32 {
        unsafe {
            outl(0xcf8, (1 << 31) | adr | (offset & 0xfc));
            inl((0xcfc + (offset & 3)) as u16)
        }
    }
    fn write8(&self, adr: u32, offset: u32, data: u8) {
        unsafe {
            outl(0xcf8, (1 << 31) | adr | (offset & 0xfc));
            outb(0xcfc + (offset & 3) as u16, data);
        }
    }
    fn write16(&self, adr: u32, offset: u32, data: u16) {
        unsafe {
            outl(0xcf8, (1 << 31) | adr | (offset & 0xfc));
            outw(0xcfc + (offset & 3) as u16, data);
        }
    }
    fn write32(&self, adr: u32, offset: u32, data: u32) {
        unsafe {
            outl(0xcf8, (1 << 31) | adr | (offset & 0xfc));
            outl(0xcfc + (offset & 3) as u16, data);
        }
    }
    fn bus_dev_fn_to_adr(&self, bus: u8, dev: u8, fnc: u8) -> u32 {
        ((bus as u32) << 16) | ((dev as u32) << 11) | ((fnc as u32) << 8)
    }
}

pub struct PCIBar {
    pub idx: u8,
    pub addr: u64,
    pub size: u64,
    pub is_io: bool,
    pub prefetchable: bool,
    pub bit64: bool,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct ExpansionROM {
    pub addr: u32,
    pub size: u32,
}

pub struct PCIDev {
    pub bus: u8,
    pub dev: u8,
    pub func: u8,

    pub bars: Vec<PCIBar>,
    pub exp_rom: Option<ExpansionROM>,
}

pub struct PCIBus {
    pub devs: Vec<PCIDev>,
    pub children: Vec<PCIBus>,

    pub self_bus: u8,
    pub sub_bus_start: u8,
    pub sub_bus_end: u8,

    pub dev: u8,
    pub func: u8,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum AddrSpaceType {
    Io,
    NonPrefetchableMem,
    PrefetchableMem,
}

fn assign_resource_recursive(
    pci: &dyn PciConfigIf,
    bus: &mut PCIBus,
    addr_type: AddrSpaceType,
    addr: &mut u64,
) {
    let bridge_align = match addr_type {
        AddrSpaceType::Io => 0x1000,                   // 4KiB
        AddrSpaceType::NonPrefetchableMem => 0x100000, // 1MiB
        AddrSpaceType::PrefetchableMem => 0x100000,    // 1MiB
    };
    *addr = align_up(*addr, bridge_align);

    let bridge_start = *addr;

    for dev in &mut bus.devs {
        let dev_adr = pci.bus_dev_fn_to_adr(dev.bus, dev.dev, dev.func);

        for bar in &mut dev.bars {
            let scan = match addr_type {
                AddrSpaceType::Io => bar.is_io,
                AddrSpaceType::PrefetchableMem => !bar.is_io && bar.prefetchable,
                AddrSpaceType::NonPrefetchableMem => !bar.is_io && !bar.prefetchable,
            };
            if !scan {
                continue;
            }

            *addr = align_up(*addr, bar.size);
            bar.addr = *addr;

            if bar.bit64 {
                pci.write32(dev_adr, 0x10 + (bar.idx as u32) * 4, *addr as u32);
                pci.write32(
                    dev_adr,
                    0x10 + (bar.idx as u32) * 4 + 4,
                    (*addr >> 32) as u32,
                );
            } else {
                pci.write32(dev_adr, 0x10 + (bar.idx) as u32 * 4, *addr as u32);
            }
            *addr += bar.size;
        }

        if addr_type == AddrSpaceType::NonPrefetchableMem {
            if let Some(exp_rom) = &mut dev.exp_rom {
                *addr = align_up(*addr, exp_rom.size as u64);
                exp_rom.addr = *addr as u32;
                pci.write32(dev_adr, 0x30, exp_rom.addr | 1); // enable
            }
        }
    }

    for child in &mut bus.children {
        assign_resource_recursive(pci, child, addr_type, addr);
    }

    *addr = align_up(*addr, bridge_align);
    let mut bridge_end = *addr - 1;

    if bridge_start == *addr {
        bridge_end = 0;
    }
    //println!(
    //    "bridge_start={:#x}, bridge_end={:#x}, *addr={:#x}",
    //    bridge_start, bridge_end, *addr
    //);
    let bridge_dev_adr = pci.bus_dev_fn_to_adr(bus.self_bus, bus.dev, bus.func);

    match addr_type {
        AddrSpaceType::Io => {
            pci.write16(
                bridge_dev_adr,
                0x1c,
                ((((bridge_end) >> 12) << 4) | ((bridge_start >> 12) << 12)) as u16,
            );
        }
        AddrSpaceType::NonPrefetchableMem => {
            let regval = ((((bridge_end) >> 20) << 20) | ((bridge_start >> 20) << 4)) as u32;

            pci.write32(bridge_dev_adr, 0x20, regval);
        }
        AddrSpaceType::PrefetchableMem => {
            let regval = ((((bridge_end) >> 20) << 20) | ((bridge_start >> 20) << 4)) as u32;
            pci.write32(bridge_dev_adr, 0x24, regval);
        }
    }
}

unsafe extern "C" {
    //static __vgabios_bin_start: u8;
}

fn enable_vga_optionrom(pci: &dyn PciConfigIf, dev: &PCIDev) {
    let adr = dev.exp_rom.unwrap().addr as *const u8;
    //let adr = &raw const __vgabios_bin_start;
    unsafe {
        let sig = *(adr as *const u16);
        if sig != 0xaa55 {
            return;
        }

        let size = (*(adr.offset(2)) as usize) * 512;
        let rom_slice = core::slice::from_raw_parts(adr, size);

        let vga_option_rom_addr = core::slice::from_raw_parts_mut(0xc0000 as *mut u8, size);

        vga_option_rom_addr.copy_from_slice(rom_slice);
        println!(
            "VGA Option ROM copied to 0xc0000-{:#x}, size={:#x}",
            0xc0000 + size,
            size
        );

        let dev_adr = pci.bus_dev_fn_to_adr(dev.bus, dev.dev, dev.func);
        pci.write32(dev_adr, 0x30, 0xc0000 | 0); // set to c0000 and disable exp_rom
    }
}

// returned true if this tree has VGA dev
fn enable_devices(pci: &dyn PciConfigIf, bus: &mut PCIBus) -> bool {
    let bridge_dev_adr = pci.bus_dev_fn_to_adr(bus.self_bus, bus.dev, bus.func);
    let mut cmd = pci.read16(bridge_dev_adr, 0x4);
    cmd |= 0x7; // enable mem, io, busmaster
    pci.write16(bridge_dev_adr, 0x4, cmd);

    let mut has_vga = false;
    for dev in &mut bus.devs {
        let dev_adr = pci.bus_dev_fn_to_adr(dev.bus, dev.dev, dev.func);
        let mut cmd = pci.read16(dev_adr, 0x4);
        cmd |= 0x7; // enable mem, io, busmaster
        pci.write16(dev_adr, 0x4, cmd);

        let class_code = pci.read16(dev_adr, 0x0a);
        if class_code == 0x0300 {
            has_vga = true;

            if dev.exp_rom.is_some() {
                enable_vga_optionrom(pci, dev);
            }
        }
    }

    for child in &mut bus.children {
        has_vga |= enable_devices(pci, child);
    }

    let mut bctrl = pci.read16(bridge_dev_adr, 0x3e);
    bctrl |= 1 << 2; // enable isa
    if has_vga {
        bctrl |= 1 << 3; // decode vga range
    }
    pci.write16(bridge_dev_adr, 0x3e, bctrl);

    has_vga
}

pub fn assign_resource(pci: &dyn PciConfigIf, root: &mut PCIBus) {
    pci.write8(0, 0x9c, 8 << 3); // 1gib

    let mut addr = 0x1000;
    assign_resource_recursive(pci, root, AddrSpaceType::Io, &mut addr);

    let mut addr = 0x90000000;
    assign_resource_recursive(pci, root, AddrSpaceType::NonPrefetchableMem, &mut addr);

    let mut addr = 0xb0000000;
    assign_resource_recursive(pci, root, AddrSpaceType::PrefetchableMem, &mut addr);

    enable_devices(pci, root);
}

pub fn scan_bus_recursive(
    pci: &dyn PciConfigIf,
    bridge_bus: u8,
    bridge_dev: u8,
    bridge_func: u8,
    bus_counter: &mut u8,
) -> PCIBus {
    let bridge_dev_adr = pci.bus_dev_fn_to_adr(bridge_bus, bridge_dev, bridge_func);
    let sub_bus = *bus_counter;
    pci.write8(bridge_dev_adr, 0x19, sub_bus as u8);
    pci.write8(bridge_dev_adr, 0x1a, 0xff); // cover all
    *bus_counter += 1;

    //println!(
    //    "scan bus={:02x}:{:02x}:{:02x}, bus_counter={}, sub_bus={}",
    //    bridge_bus, bridge_dev, bridge_func, *bus_counter, sub_bus
    //);

    let mut children = Vec::new();
    let mut devs = Vec::new();
    for dev in 0..32 {
        let dev_adr = pci.bus_dev_fn_to_adr(sub_bus as u8, dev, 0);
        let vid = pci.read16(dev_adr, 0);
        //println!(
        //    "scan bus.dev: bus={:02x}:{:02x}:{:02x}, dev={:02x}:{:02x}",
        //    bridge_bus, bridge_dev, bridge_func, sub_bus, dev
        //);

        if vid == 0xffff {
            continue;
        }
        let headertyp = pci.read8(dev_adr, 0x0e);
        let num_func = if (headertyp & 0x80) != 0 { 8 } else { 1 };
        for func in 0..num_func {
            let dev_adr = pci.bus_dev_fn_to_adr(sub_bus as u8, dev, func);
            //println!(
            //    "scan bus.dev.func: bus={:02x}:{:02x}:{:02x}, dev={:02x}:{:02x}:{:02x}",
            //    bridge_bus, bridge_dev, bridge_func, sub_bus, dev, func
            //);

            let vid = pci.read16(dev_adr, 0);
            if vid == 0xffff {
                continue;
            }
            let is_bridge = (headertyp & 1) == 1;

            if is_bridge {
                let child =
                    scan_bus_recursive(pci, sub_bus as u8, dev as u8, func as u8, bus_counter);

                children.push(child);
            } else {
                let mut bar = 0;
                let mut bars = Vec::new();

                while bar < 6 {
                    pci.write32(dev_adr, 0x10 + bar * 4, 0xfffffff0);
                    let b0 = pci.read32(dev_adr, 0x10 + bar * 4);
                    let this_bar = bar;
                    bar += 1;

                    let prefetchable = (b0 & (1 << 3)) != 0;
                    let is_io = (b0 & 1) != 0;

                    if b0 & (1 << 2) != 0 {
                        // 64bit
                        bar += 1;
                        if (b0 & 0xffff_fff0) == 0 {
                            continue;
                        }

                        pci.write32(dev_adr, 0x10 + bar * 4, 0xffff_ffff);
                        let b1 = pci.read32(dev_adr, 0x10 + bar * 4);
                        //println!(
                        //    "BAR[{}] : b0={:#x}, b1={:#x} prefetchable={}, is_io={}, bus={:#x}, dev={:#x}, func={:#x}",
                        //    this_bar, b0, b1, prefetchable, is_io, sub_bus, dev, func
                        //);

                        let addr = ((b1 as u64) << 32) | (b0 as u64);
                        let size: u64 =
                            ((addr & 0x0000_0000_ffff_fff0) as u64 ^ 0x0000_0000_ffff_ffff) + 1;
                        bars.push(PCIBar {
                            idx: this_bar as u8,
                            addr,
                            size: size.into(),
                            is_io,
                            prefetchable,
                            bit64: true,
                        });
                    } else {
                        if (b0 & 0xffff_fff0) == 0 {
                            continue;
                        }

                        let addr = b0 as u64;
                        let size = if is_io {
                            ((b0 & 0x0000_fff0) ^ 0x0000_ffff) + 1
                        } else {
                            ((b0 & 0xffff_fff0) ^ 0xffff_ffff) + 1
                        };
                        bars.push(PCIBar {
                            idx: this_bar as u8,
                            addr,
                            size: size.into(),
                            is_io,
                            prefetchable,
                            bit64: false,
                        });
                    }
                }

                pci.write32(dev_adr, 0x30, 0xffffffff);
                let exp_rom_val = pci.read32(dev_adr, 0x30);
                let mut exp_rom = None;
                if exp_rom_val != 0 {
                    exp_rom = Some(ExpansionROM {
                        addr: 0,
                        size: ((exp_rom_val & 0xffff_fff0) ^ 0xffff_ffff) + 1,
                    });
                }

                devs.push(PCIDev {
                    bus: sub_bus as u8,
                    dev,
                    func,
                    bars,
                    exp_rom,
                });
            }
        }
    }
    //println!(
    //    "scan bus end={:02x}:{:02x}:{:02x}, [{:02x}]-[{:02x}]",
    //    bridge_bus,
    //    bridge_dev,
    //    bridge_func,
    //    sub_bus,
    //    (*bus_counter) - 1
    //);

    pci.write8(bridge_dev_adr, 0x1a, *bus_counter - 1); // cover sub
    return PCIBus {
        devs,
        children,
        self_bus: bridge_bus as u8,
        sub_bus_start: sub_bus as u8,
        sub_bus_end: *bus_counter as u8,
        dev: bridge_dev,
        func: bridge_func,
    };
}

pub fn scan_bus(pci: &dyn PciConfigIf, bus: u32, bridge_dev: u8, bridge_func: u8) -> PCIBus {
    pci.write8(0, 0x19, 0);
    pci.write8(0, 0x1a, 0xff); // cover all

    let mut bus_counter = 0;

    scan_bus_recursive(pci, bus as u8, bridge_dev, bridge_func, &mut bus_counter)
}

pub fn show_pci(root: &PCIBus, pci: &dyn PciConfigIf) {
    println!(
        "Bus {:02x}:{:02x}:{:02x} {:02x}-{:02x}:",
        root.self_bus, root.dev, root.func, root.sub_bus_start, root.sub_bus_end
    );

    let bridge_dev_adr = pci.bus_dev_fn_to_adr(root.self_bus, root.dev, root.func);

    let membase_regval = pci.read32(bridge_dev_adr, 0x20);
    let membase_start = (membase_regval & 0x0000fff0) << 16;
    let membase_end = (membase_regval & 0xfff00000) + (1024 * 1024) - 1;

    let pref_membase_regval = pci.read32(bridge_dev_adr, 0x24);
    let pref_membase_start = (pref_membase_regval & 0x0000fff0) << 16;
    let pref_membase_end = (pref_membase_regval & 0xfff00000) + (1024 * 1024) - 1;

    let iobase = pci.read16(bridge_dev_adr, 0x1c);
    let brctl = pci.read16(bridge_dev_adr, 0x3e);

    println!(
        "{:02x}:{:02x}:{:02x} membase:{:#08x}-{:#08x}, pref_membase:{:#08x}-{:#08x}, iobase:{:04x}, brctl:{:04x}",
        root.self_bus,
        root.dev,
        root.func,
        membase_start,
        membase_end,
        pref_membase_start,
        pref_membase_end,
        iobase,
        brctl
    );

    for d in &root.devs {
        let devid = pci.read16(pci.bus_dev_fn_to_adr(d.bus, d.dev, d.func), 0x02);
        let vid = pci.read16(pci.bus_dev_fn_to_adr(d.bus, d.dev, d.func), 0x00);

        println!(
            "{:02x}:{:02x}:{:02x}, vendor={:04x}, dev={:04x}",
            d.bus, d.dev, d.func, vid, devid
        );
        for bar in &d.bars {
            println!(
                "BAR[{}] : addr={:08x} size={:08x}, is_io={}, prefetchable={}",
                bar.idx, bar.addr, bar.size, bar.is_io, bar.prefetchable
            );
        }
        if let Some(exp_rom) = &d.exp_rom {
            println!(
                "EXP_ROM: addr={:08x} size={:08x}",
                exp_rom.addr, exp_rom.size
            );
        }
    }
    for b in &root.children {
        show_pci(b, pci);
    }
}
