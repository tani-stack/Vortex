//! Timer abstraction for delays and timed operations

use vortex_types::VortexResult;

/// Timer trait
pub trait Timer: Send + Sync {
    fn delay_ms(&self, ms: u32) -> VortexResult<()>;
    fn delay_us(&self, us: u32) -> VortexResult<()>;
    fn get_ticks(&self) -> u64;
}

/// ARM64 Timer implementation
pub struct Arm64Timer;

impl Timer for Arm64Timer {
    fn delay_ms(&self, ms: u32) -> VortexResult<()> {
        self.delay_us(ms * 1000)?;
        Ok(())
    }

    fn delay_us(&self, us: u32) -> VortexResult<()> {
        // Busy wait (not ideal but works for QEMU)
        for _ in 0..us {
            core::hint::spin_loop();
        }
        Ok(())
    }

    fn get_ticks(&self) -> u64 {
        // Read ARM generic timer
        0
    }
}

pub fn init() -> VortexResult<()> {
    Ok(())
}
