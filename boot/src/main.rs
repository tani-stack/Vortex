#![no_std]
#![no_main]
#![feature(abi_efiapi)]
mod tpm; mod loader; mod uefi;
use tpm::{TpmDevice, SpiTpm};
use core::panic::PanicInfo;
const EXPECTED_ID: [u8; 32] = *b"AERO-RUST-PROD-KEY-V1-0000000000";
#[panic_handler] fn panic(_: &PanicInfo) -> ! { loop { unsafe { core::arch::asm!("wfe") } } }
fn verify_hardware_signature(tpm: &mut dyn TpmDevice) -> Result<bool, tpm::TpmError> {
    let ek = tpm.read_endorsement_key()?;
    let mf = tpm.read_signed_manifest()?;
    if mf.product_id != EXPECTED_ID { return Ok(false); }
    tpm.verify_signature(&ek, &mf)
}
#[no_mangle] pub extern "efiapi" fn efi_main(_h: uefi::Handle, st: *mut uefi::SystemTable) -> uefi::Status {
    uefi::init_console(st);
    let mut tpm = SpiTpm::new();
    match verify_hardware_signature(&mut tpm) {
        Ok(true) => { uefi::println!("[BOOT] TPM OK, jumping to kernel"); unsafe { loader::load_and_jump_to_kernel() } },
        _ => { uefi::println!("[BOOT] HALT: Invalid Signature 0xA1"); loop { unsafe { core::arch::asm!("wfe") } } }
    }
}
