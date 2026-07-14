use spin::Mutex;
pub static PICS: Mutex<u8> = Mutex::new(0);
pub fn init() {
    #[cfg(target_arch="x86_64")] { load_idt_x86(); }
    #[cfg(target_arch="aarch64")] { unsafe { core::arch::asm!("msr VBAR_EL1, {0}", in(reg) 0x40080000u64); core::arch::asm!("msr DAIFClr, #2"); } }
}
#[cfg(target_arch="x86_64")] fn load_idt_x86() {}
