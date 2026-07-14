pub fn idle_task() -> ! { loop { unsafe { core::arch::asm!("wfi") } } }
