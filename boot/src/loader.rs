pub unsafe fn load_and_jump_to_kernel() -> ! {
    // In real HW: parse ELF from ESP partition, map via MMU, set VBAR_EL1
    core::arch::asm!("mov x0, #0", "b .", options(noreturn));
}
