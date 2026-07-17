#![no_std]
#![no_main]
#![feature(abi_efiapi)]

mod loader;
mod tpm;
mod uefi;

use core::panic::PanicInfo;
use tpm::{SpiTpm, TpmDevice};

const EXPECTED: [u8; 32] = *b"VORTEX-RUST-PROD-KEY-V2-000000";

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

fn ct_eq(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let mut d = 0u8;
    for i in 0..32 {
        d |= a[i] ^ b[i]
    }
    d == 0
}

#[no_mangle]
pub extern "efiapi" fn efi_main(_h: uefi::Handle, st: *mut uefi::SystemTable) -> uefi::Status {
    uefi::init_console(st);
    let mut tpm = SpiTpm::new(0);
    let _ = tpm.self_test();
    if let Ok(mf) = tpm.read_signed_manifest() {
        if !ct_eq(&mf.product_id, &EXPECTED) {
            loop {
                unsafe {
                    core::arch::asm!("wfi");
                }
            }
        }
    }
    let base = 0x5000_0000 as *const u8;
    unsafe { loader::load_and_jump_to_kernel(base, 4 * 1024 * 1024) }
}
