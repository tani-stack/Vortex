#![no_std]
#![feature(abi_x86_interrupt)]
extern crate alloc;
pub mod idt; pub mod gdt; pub mod mem; pub mod sched; pub mod ipc; pub mod sync;
use linked_list_allocator::LockedHeap;
#[global_allocator] static ALLOCATOR: LockedHeap = LockedHeap::empty();
pub fn init() { mem::init_frame_allocator(); unsafe { ALLOCATOR.lock().init(0x4444_4444_0000 as *mut u8, 1024*1024); } idt::init(); ipc::init(); sched::init(); }
#[no_mangle] pub extern "C" fn _start() -> ! { init(); loop { core::hint::spin_loop(); } }
