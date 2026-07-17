//! SPI (Serial Peripheral Interface) Hardware Abstraction

use vortex_types::VortexResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SPIMode {
    Mode0,
    Mode1,
    Mode2,
    Mode3,
}

pub struct SPIMaster {
    bus: u8,
    speed_hz: u32,
    mode: SPIMode,
}

impl SPIMaster {
    pub fn new(bus: u8, speed_hz: u32, mode: SPIMode) -> VortexResult<Self> {
        Self::init_spi(bus, speed_hz, mode)?;
        Ok(Self { bus, speed_hz, mode })
    }

    pub fn transfer(&self, write_buf: &[u8], read_buf: &mut [u8]) -> VortexResult<usize> {
        Self::spi_transfer(self.bus, write_buf, read_buf)
    }

    pub fn write(&self, data: &[u8]) -> VortexResult<()> {
        let mut dummy = [0u8; 256];
        let read_size = data.len().min(256);
        Self::spi_transfer(self.bus, data, &mut dummy[..read_size])?;
        Ok(())
    }

    pub fn read(&self, buf: &mut [u8]) -> VortexResult<usize> {
        let dummy = [0u8; 256];
        let read_size = buf.len().min(256);
        Self::spi_transfer(self.bus, &dummy[..read_size], &mut buf[..read_size])
    }

    pub fn set_speed(&mut self, speed_hz: u32) -> VortexResult<()> {
        Self::spi_set_speed(self.bus, speed_hz)?;
        self.speed_hz = speed_hz;
        Ok(())
    }

    #[inline(always)]
    fn init_spi(bus: u8, _speed_hz: u32, _mode: SPIMode) -> VortexResult<()> {
        let _ = bus;
        Ok(())
    }

    #[inline(always)]
    fn spi_transfer(bus: u8, _write_buf: &[u8], _read_buf: &mut [u8]) -> VortexResult<usize> {
        let _ = bus;
        Ok(0)
    }

    #[inline(always)]
    fn spi_set_speed(bus: u8, _speed: u32) -> VortexResult<()> {
        let _ = bus;
        Ok(())
    }
}

pub fn init() {}
