#![no_std]

extern crate init86;
use x86::io::{inb, outb};

pub use init86::X86State;
use linked_list_allocator::LockedHeap;

static ALLOCATOR_16: LockedHeap = LockedHeap::empty();

unsafe extern "C" {
    pub fn reset();
    pub fn enter_to_16_asm();
    static mut state16_regs_flat32: X86State;
}

pub fn keep_syms() {
    unsafe {
        reset();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn enter_to_16() {
    unsafe {
        enter_to_16_asm();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn set_16state(st: &X86State) {
    unsafe {
        let p = &raw mut state16_regs_flat32 as *mut X86State;
        *p = *st;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_16state() -> X86State {
    unsafe {
        let p = &raw const state16_regs_flat32 as *const X86State;
        *p
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn alloc_from_16(size: usize) -> *mut u8 {
    let mut heap16 = ALLOCATOR_16.lock();
    let ptr = heap16
        .allocate_first_fit(core::alloc::Layout::from_size_align(size, 8).unwrap())
        .unwrap();
    ptr.as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn free_to_16(ptr: *mut u8, size: usize) {
    unsafe {
        ALLOCATOR_16.lock().deallocate(
            core::ptr::NonNull::new(ptr).unwrap(),
            core::alloc::Layout::from_size_align(size, 8).unwrap(),
        );
    }
}

unsafe extern "C" {
    static mut __end16_flat32: u8;
    static mut __end16_in_segment: u8;
    static mut int_handler_0h: u8;
    static mut int_handler_4h: u8;
    static mut int_handler_10h: u8;
    static int_number_flat32: u32;
}

fn install_ivt(int_num: u8, handler: u16) {
    let ivt_addr = 0x0000 as *mut u16;
    unsafe {
        core::ptr::write_volatile(ivt_addr.offset(int_num as isize * 2), handler);
        core::ptr::write_volatile(ivt_addr.offset(int_num as isize * 2 + 1), 0xf000);
    }
}

static mut INT_HANDLER_TABLE: [Option<unsafe extern "C" fn()>; 256] = [None; 256];

#[unsafe(no_mangle)]
pub extern "C" fn handle_exceptions() {
    unsafe {
        let fp = INT_HANDLER_TABLE[int_number_flat32 as usize];
        if let Some(fp) = fp {
            fp();
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn install_int_handler(fptr: unsafe extern "C" fn(), int_num: usize) {
    unsafe {
        INT_HANDLER_TABLE[int_num] = Some(fptr);
    }
}

pub unsafe extern "C" fn ignore_interrupt() {
    unsafe {
        let int_num = int_number_flat32;

        match int_num {
            0x0 => {
                // handle 8254 timer
                outb(0x20, 0x20);
            }

            0x4 => {
                // handle uart rx ready
                let _ = inb(0x3f8);
                outb(0x20, 0x20);
            }

            0x10 => {
                // handle vga dummy
            }
            _ => {}
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn flashrom_init86_rs_init() {
    // Initialize the heap allocator for real mode
    unsafe {
        let heap16_start: *mut u8 = &raw mut __end16_flat32;
        let heap16_size: usize = 32 * 1024 as usize;

        if heap16_size > 0 {
            ALLOCATOR_16.lock().init(heap16_start, heap16_size);
        }
    }

    /* initialize exceptions */
    install_ivt(0x0, (&raw const int_handler_0h) as u16);
    install_ivt(0x4, (&raw const int_handler_4h) as u16);
    install_ivt(0x10, (&raw const int_handler_10h) as u16);

    install_int_handler(ignore_interrupt, 0);
    install_int_handler(ignore_interrupt, 0x4);
    install_int_handler(ignore_interrupt, 0x10);

    unsafe {
        outb(0x20, 0x11); // ICW1
        outb(0xa0, 0x11); // ICW1
        outb(0x21, 0x00); // ICW2
        outb(0xa1, 0x08); // ICW2, offset=8
        outb(0x21, 0x04); // ICW3, 2=slave
        outb(0xa1, 0x02); // ICW3, cascade to 2
        outb(0x21, 0x01); // ICW4, 8086
        outb(0xa1, 0x01); // ICW4, 8086

        outb(0x21, 0xee); // OCW1, enable irq 0
        outb(0xa1, 0xff); // OCW1, disable all

        outb(0x43, 0x34); // start PIT
        outb(0x40, 0);
        outb(0x40, 0);
    }
}
