#![no_main]
#![no_std]

extern crate alloc;

extern crate common;
use common::pci;
use common::println;
use common::uart;

extern crate flashrom_init86;
use flashrom_init86 as init86;
//extern crate init86;
fn invoke_int10(regs: &mut init86::X86State) {
    unsafe {
        let service_table = init86::get_service_func_table();
        let ptr = common::alloc_from_16(0x100);
        {
            *ptr.offset(0) = 0xcd;
            *ptr.offset(1) = 0x10;
            *ptr.offset(2) = 0xcb; // retf
        }
        println!("ptr = {:?}", ptr);

        regs.cs = (ptr as u32 / 16) & 0xf000;
        regs.eip = ptr as u32 % 65536;
        regs.ds = regs.cs;
        regs.ss = regs.cs;
        regs.esp = 0xfffc;

        ((*service_table).set_16state)(regs);
        ((*service_table).enter_to_16)();
        *regs = ((*service_table).get_16state)();
        println!("ok");

        common::free_to_16(ptr, 0x100);
        println!("ok2");
    }
}

fn find_vga<'a>(bus: &'a pci::PCIBus, pci: &dyn pci::PciConfigIf) -> Option<&'a pci::PCIDev> {
    for dev in &bus.devs {
        let dev_adr = pci.bus_dev_fn_to_adr(dev.bus, dev.dev, dev.func);
        let class_code = pci.read16(dev_adr, 0x0a);
        if class_code == 0x0300 {
            let vendor_id = pci.read16(dev_adr, 0);
            let device_id = pci.read16(dev_adr, 2);
            let class_code = pci.read8(dev_adr, 0x0b);
            let subclass_code = pci.read8(dev_adr, 0x0a);
            let prog_if = pci.read8(dev_adr, 0x09);

            println!(
                "VGA {:02x}:{:02x}:{:02x}: {:04x}:{:04x} {:02x}:{:02x}:{:02x}",
                dev.bus,
                dev.dev,
                dev.func,
                vendor_id,
                device_id,
                class_code,
                subclass_code,
                prog_if
            );
            return Some(dev);
        }
    }

    for child in &bus.children {
        let r = find_vga(child, pci);
        if r.is_some() {
            return r;
        }
    }

    None
}

unsafe extern "C" {
    //static mut vgabios_start: u8;
    //static mut vgabios_end: u8;
}

#[repr(C, packed)]
#[allow(non_snake_case)]
struct VgaBiosHeader {
    VbeSignature: [u8; 4],
    VbeVersion: u16,
    OemStringPtr: [u16; 2],
    Capabilities: [u8; 4],
    VideoModePtr: [u16; 2],
    TotalMemory: u16,
    OemSoftwareRev: u16,
    OemVendorNamePtr: u32,
    OemProductNamePtr: u32,
    OemProductRevPtr: u32,
    Reserved: [u8; 222],
    OemData: [u8; 256],
}

pub fn main() {
    println!("Hello test_vga!!");

    let pciif = common::pci::IOPciConfig {};

    let mut pci = common::pci::scan_bus(&pciif, 0, 0, 0);
    common::pci::assign_resource(&pciif, &mut pci);
    common::pci::show_pci(&pci, &pciif);

    let vga = find_vga(&pci, &pciif);

    if let Some(vga) = vga {
        let mut st = unsafe { core::mem::zeroed::<init86::X86State>() };
        // init optionrom
        st.cs = 0xc000;
        st.eip = 0x0003;
        st.esp = 0xfffc;

        st.ss = 0xf000;
        st.ds = 0xf000;
        st.es = 0xf000;

        st.ebx = 0xffff;
        st.edx = 0xffff;
        st.edi = 0;

        st.eax = ((vga.bus as u32) << 8) | ((vga.dev as u32) << 3) | (vga.func as u32);

        println!("invoke vga option rom");
        let service_table = init86::get_service_func_table();
        unsafe {
            ((*service_table).set_16state)(&st);
            ((*service_table).enter_to_16)();
        }

        println!("Returned from vga option rom = {:x}", st.eax);

        // get video mode
        st.esp = 0xfffc;
        st.ss = 0xf000;

        let vbe_info = common::alloc_from_16t::<VgaBiosHeader>();

        st.eax = 0x4f00;
        st.es = (vbe_info as u32 / 16) & 0xf000;
        st.edi = vbe_info as u32 % 65536;

        unsafe {
            (*vbe_info).VbeSignature[0] = b'V';
            (*vbe_info).VbeSignature[1] = b'B';
            (*vbe_info).VbeSignature[2] = b'E';
            (*vbe_info).VbeSignature[3] = b'2';
        }

        invoke_int10(&mut st);

        println!("get vbe info {:x}", st.eax);

        st.eax = 0x4f02;
        st.ebx = 0x3; // set mode = 0x3
        invoke_int10(&mut st);

        unsafe {
            println!(
                "modes_ptr = {:x} {:x}",
                core::ptr::read_unaligned(&raw const (*vbe_info).VideoModePtr[0]),
                core::ptr::read_unaligned(&raw const (*vbe_info).VideoModePtr[1])
            );

            let mut i = 0;
            let off = core::ptr::read_unaligned(&raw const (*vbe_info).VideoModePtr[0]) as usize;
            let seg = core::ptr::read_unaligned(&raw const (*vbe_info).VideoModePtr[1]) as usize;
            let ptr = (seg * 16 + off) as *const u16;

            loop {
                let mode_val = *ptr.offset(i);
                if mode_val == 0xffff {
                    break;
                }
                println!("mode[{}] = {}", i, mode_val);
                i += 1;
            }
        }

        println!("set mode to 0x003 = {:x}", st.eax);

        unsafe {
            let ptr = 0xb8000 as *mut u8;
            *ptr = b'a';
        }

        common::free_to_16t(vbe_info);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rmain() {
    main();
}

#[unsafe(link_section = ".text.start")]
#[unsafe(no_mangle)]
extern "C" fn _start() -> i32 {
    common::common_init_from_sdram();

    unsafe {
        main();
    }
    0
}
