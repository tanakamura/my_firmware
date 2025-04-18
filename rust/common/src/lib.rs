#![no_std]

pub mod pci;
pub mod uart;

extern crate alloc;
extern crate init86;

use core::panic::PanicInfo;

use linked_list_allocator::LockedHeap;
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

unsafe extern "C" {
    static mut __BSS_RAM_START: u8;
    static mut __end: u8;
    static mut __ram_last: u8;
    static mut __end16_flat32: u8;
    static mut __end16_in_segment: u8;
}

fn clear_bss() {
    /* clear bss */
    let p0 = &raw mut __BSS_RAM_START;
    let p1 = &raw mut __end;
    let len = p1 as usize - p0 as usize;
    unsafe {
        core::ptr::write_bytes(p0, 0, len);
    }
}
pub fn common_init_from_sdram() {
    clear_bss();
    init_heap32();
}

fn init_heap32() {
    unsafe {
        let heap_start: *mut u8 = &raw mut __end;
        let heap_size: usize = &raw mut __ram_last as usize - &raw mut __end as usize;
        ALLOCATOR.lock().init(heap_start, heap_size);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn common_init_from_flash() {
    init_heap32();
}

pub fn alloc_from_16(size: usize) -> *mut u8 {
    let service_func = init86::get_service_func_table();
    unsafe { ((*service_func).alloc_from_16)(size) }
}

pub fn free_to_16(ptr: *mut u8, size: usize) {
    let service_func = init86::get_service_func_table();
    unsafe {
        ((*service_func).free_to_16)(ptr, size);
    }
}

pub fn alloc_from_16t<T>() -> *mut T {
    let size = core::mem::size_of::<T>();
    alloc_from_16(size) as *mut T
}

pub fn free_to_16t<T>(ptr: *mut T) {
    let size = core::mem::size_of::<T>();
    free_to_16(ptr as *mut u8, size);
}
