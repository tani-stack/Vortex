//! GPIO (General Purpose Input/Output) abstraction

use vortex_types::VortexResult;
use super::mmio;

/// GPIO Pin trait
pub trait GpioPin: Send + Sync {
    fn set_high(&mut self) -> VortexResult<()>;
    fn set_low(&mut self) -> VortexResult<()>;
    fn read(&self) -> VortexResult<bool>;
    fn set_direction(&mut self, is_output: bool) -> VortexResult<()>;
}

/// ARM64 GPIO implementation (QEMU/BCM2711)
pub struct Arm64Gpio {
    pin: u32,
    is_output: bool,
}

impl Arm64Gpio {
    pub fn new(pin: u32) -> Self {
        Self {
            pin,
            is_output: false,
        }
    }

    pub fn as_output(pin: u32) -> Self {
        Self {
            pin,
            is_output: true,
        }
    }

    pub fn as_input(pin: u32) -> Self {
        Self {
            pin,
            is_output: false,
        }
    }
}

impl GpioPin for Arm64Gpio {
    fn set_high(&mut self) -> VortexResult<()> {
        if !self.is_output {
            return Err(vortex_types::VortexError::HardwareError);
        }
        let pin_offset = (self.pin / 32) * 4;
        let bit = 1u32 << (self.pin % 32);
        mmio::mmio_write(mmio::GPIO_BASE + pin_offset, bit);
        Ok(())
    }

    fn set_low(&mut self) -> VortexResult<()> {
        if !self.is_output {
            return Err(vortex_types::VortexError::HardwareError);
        }
        let pin_offset = (self.pin / 32) * 4 + 0x28;  // Clear register offset
        let bit = 1u32 << (self.pin % 32);
        mmio::mmio_write(mmio::GPIO_BASE + pin_offset, bit);
        Ok(())
    }

    fn read(&self) -> VortexResult<bool> {
        let pin_offset = (self.pin / 32) * 4;
        let bit = 1u32 << (self.pin % 32);
        let value = mmio::mmio_read(mmio::GPIO_BASE + pin_offset);
        Ok((value & bit) != 0)
    }

    fn set_direction(&mut self, is_output: bool) -> VortexResult<()> {
        self.is_output = is_output;
        // On real hardware, would configure GPIO registers here
        Ok(())
    }
}

pub fn init() -> VortexResult<()> {
    // Initialize GPIO subsystem
    Ok(())
}
