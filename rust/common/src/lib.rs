#![no_std]

pub mod pci;
pub mod uart;

extern crate alloc;

use core::mem::MaybeUninit;
use core::panic::PanicInfo;

use linked_list_allocator::LockedHeap;
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();
static ALLOCATOR_16: LockedHeap = LockedHeap::empty();

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

pub fn clear_bss() {
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
        println!(
            "heap_start = {:x} heap_size = {:x}",
            heap_start as usize, heap_size
        );

        ALLOCATOR.lock().init(heap_start, heap_size);

        println!("heap init done");
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn common_init_from_flash() {
    init_heap32();
    // Initialize the heap allocator for real mode
    unsafe {
        let heap16_start: *mut u8 = &raw mut __end16_flat32;
        let heap16_size: usize = 64 * 1024 - &raw const __end16_in_segment as usize;

        if heap16_size > 0 {
            ALLOCATOR_16.lock().init(heap16_start, heap16_size);
        }
    }
}

pub fn alloc_from_16(size: usize) -> *mut u8 {
    unsafe {
        let mut heap16 = ALLOCATOR_16.lock();
        let ptr = heap16
            .allocate_first_fit(core::alloc::Layout::from_size_align(size, 8).unwrap())
            .unwrap();
        ptr.as_ptr()
    }
}

pub fn alloc_from_16t<T>() -> *mut T {
    let size = core::mem::size_of::<T>();
    alloc_from_16(size) as *mut T
}

pub fn free_to_16(ptr: *mut u8, size: usize) {
    unsafe {
        ALLOCATOR_16.lock().deallocate(
            core::ptr::NonNull::new(ptr).unwrap(),
            core::alloc::Layout::from_size_align(size, 8).unwrap(),
        );
    }
}

pub fn free_to_16t<T>(ptr: *mut T) {
    let size = core::mem::size_of::<T>();
    free_to_16(ptr as *mut u8, size);
}
