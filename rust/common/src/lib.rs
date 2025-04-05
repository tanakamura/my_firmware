#![no_std]

pub mod pci;
pub mod uart;

extern crate alloc;

use core::mem::MaybeUninit;
use core::panic::PanicInfo;

use linked_list_allocator::LockedHeap;
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn common_init() {
    // Initialize the heap allocator
    unsafe {
        let heap_region: &'static mut [MaybeUninit<u8>] =
            core::slice::from_raw_parts_mut(0x1000_0000 as *mut MaybeUninit<u8>, 0x1000_0000);
        ALLOCATOR.lock().init_from_slice(heap_region);
    }
}
