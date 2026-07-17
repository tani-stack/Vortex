//! I2C (Inter-Integrated Circuit) Hardware Abstraction

use vortex_types::VortexResult;

pub struct I2CMaster {
    bus: u8,
    speed_hz: u32,
}

impl I2CMaster {
    pub fn new(bus: u8, speed_hz: u32) -> VortexResult<Self> {
        Self::init_i2c(bus, speed_hz)?;
        Ok(Self { bus, speed_hz })
    }

    pub fn write(&self, addr: u8, data: &[u8]) -> VortexResult<()> {
        Self::i2c_write(self.bus, addr, data)?;
        Ok(())
    }

    pub fn read(&self, addr: u8, buf: &mut [u8]) -> VortexResult<usize> {
        Self::i2c_read(self.bus, addr, buf)
    }

    pub fn write_then_read(&self, addr: u8, write_data: &[u8], read_buf: &mut [u8]) -> VortexResult<usize> {
        Self::i2c_write(self.bus, addr, write_data)?;
        Self::i2c_read(self.bus, addr, read_buf)
    }

    pub fn write_reg(&self, addr: u8, reg: u8, value: u8) -> VortexResult<()> {
        let data = [reg, value];
        Self::i2c_write(self.bus, addr, &data)?;
        Ok(())
    }

    pub fn read_reg(&self, addr: u8, reg: u8) -> VortexResult<u8> {
        let mut buf = [0u8; 1];
        Self::i2c_write(self.bus, addr, &[reg])?;
        Self::i2c_read(self.bus, addr, &mut buf)?;
        Ok(buf[0])
    }

    pub fn set_speed(&mut self, speed_hz: u32) -> VortexResult<()> {
        Self::i2c_set_speed(self.bus, speed_hz)?;
        self.speed_hz = speed_hz;
        Ok(())
    }

    #[inline(always)]
    fn init_i2c(bus: u8, _speed_hz: u32) -> VortexResult<()> {
        let _ = bus;
        Ok(())
    }

    #[inline(always)]
    fn i2c_write(bus: u8, _addr: u8, _data: &[u8]) -> VortexResult<()> {
        let _ = bus;
        Ok(())
    }

    #[inline(always)]
    fn i2c_read(bus: u8, _addr: u8, _buf: &mut [u8]) -> VortexResult<usize> {
        let _ = bus;
        Ok(0)
    }

    #[inline(always)]
    fn i2c_set_speed(bus: u8, _speed: u32) -> VortexResult<()> {
        let _ = bus;
        Ok(())
    }
}

pub fn init() {}
