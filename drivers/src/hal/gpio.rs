//! GPIO (General Purpose Input/Output) Hardware Abstraction

use vortex_types::{VortexError, VortexResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GPIOMode {
    Input,
    Output,
    Alternate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GPIOLevel {
    Low,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PullMode {
    NoPull,
    PullUp,
    PullDown,
}

pub struct GPIO {
    pin: u8,
    mode: GPIOMode,
}

impl GPIO {
    pub fn new(pin: u8) -> Self {
        Self {
            pin,
            mode: GPIOMode::Input,
        }
    }

    pub fn set_mode(&mut self, mode: GPIOMode) -> VortexResult<()> {
        self.mode = mode;
        Self::configure_pin(self.pin, mode)?;
        Ok(())
    }

    pub fn set_level(&self, level: GPIOLevel) -> VortexResult<()> {
        if self.mode != GPIOMode::Output {
            return Err(VortexError::InvalidParameter);
        }
        Self::write_pin(self.pin, level)?;
        Ok(())
    }

    pub fn get_level(&self) -> VortexResult<GPIOLevel> {
        Self::read_pin(self.pin)
    }

    pub fn set_pull(&mut self, pull: PullMode) -> VortexResult<()> {
        Self::configure_pull(self.pin, pull)?;
        Ok(())
    }

    #[inline(always)]
    fn configure_pin(pin: u8, mode: GPIOMode) -> VortexResult<()> {
        let _ = pin;
        let _ = mode;
        Ok(())
    }

    #[inline(always)]
    fn write_pin(pin: u8, level: GPIOLevel) -> VortexResult<()> {
        let _ = pin;
        let _ = level;
        Ok(())
    }

    #[inline(always)]
    fn read_pin(pin: u8) -> VortexResult<GPIOLevel> {
        let _ = pin;
        Ok(GPIOLevel::Low)
    }

    #[inline(always)]
    fn configure_pull(pin: u8, pull: PullMode) -> VortexResult<()> {
        let _ = pin;
        let _ = pull;
        Ok(())
    }
}

pub fn init() {}
// Hardware Abstraction Layer for GPIO
