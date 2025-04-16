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
    static mut __end: u8;
    static mut __ram_last: u8;
    static mut __end16_flat32: u8;
    static mut __end16_in_segment: u8;

}

#[unsafe(no_mangle)]
pub extern "C" fn common_init() {
    // Initialize the heap allocator
    unsafe {
        let heap_start: *mut u8 = &raw mut __end;
        let heap_size: usize = &raw mut __ram_last as usize - &raw mut __end as usize;

        ALLOCATOR.lock().init(heap_start, heap_size);

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

pub fn free_to_16(ptr: *mut u8, size: usize) {
    unsafe {
        ALLOCATOR_16.lock().deallocate(
            core::ptr::NonNull::new(ptr).unwrap(),
            core::alloc::Layout::from_size_align(size, 8).unwrap(),
        );
    }
}
