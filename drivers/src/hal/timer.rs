//! Timer Hardware Abstraction

use vortex_types::VortexResult;

pub struct Timer {
    id: u8,
    frequency_hz: u32,
}

impl Timer {
    pub fn new(id: u8, frequency_hz: u32) -> VortexResult<Self> {
        Self::init_timer(id, frequency_hz)?;
        Ok(Self { id, frequency_hz })
    }

    pub fn start(&self) -> VortexResult<()> {
        Self::timer_start(self.id)?;
        Ok(())
    }

    pub fn stop(&self) -> VortexResult<()> {
        Self::timer_stop(self.id)?;
        Ok(())
    }

    pub fn reset(&self) -> VortexResult<()> {
        Self::timer_reset(self.id)?;
        Ok(())
    }

    pub fn get_count(&self) -> VortexResult<u32> {
        Self::timer_get_count(self.id)
    }

    pub fn set_period(&mut self, period_us: u32) -> VortexResult<()> {
        Self::timer_set_period(self.id, period_us)?;
        Ok(())
    }

    pub fn set_frequency(&mut self, frequency_hz: u32) -> VortexResult<()> {
        let period_us = 1_000_000 / frequency_hz;
        Self::timer_set_period(self.id, period_us)?;
        Ok(())
    }

    #[inline(always)]
    fn init_timer(id: u8, _freq: u32) -> VortexResult<()> {
        let _ = id;
        Ok(())
    }

    #[inline(always)]
    fn timer_start(id: u8) -> VortexResult<()> {
        let _ = id;
        Ok(())
    }

    #[inline(always)]
    fn timer_stop(id: u8) -> VortexResult<()> {
        let _ = id;
        Ok(())
    }

    #[inline(always)]
    fn timer_reset(id: u8) -> VortexResult<()> {
        let _ = id;
        Ok(())
    }

    #[inline(always)]
    fn timer_get_count(id: u8) -> VortexResult<u32> {
        let _ = id;
        Ok(0)
    }

    #[inline(always)]
    fn timer_set_period(id: u8, _period_us: u32) -> VortexResult<()> {
        let _ = id;
        Ok(())
    }
}

pub fn init() {}
