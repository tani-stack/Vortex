pub trait I2cBus { fn write(&mut self, addr: u8, data: &[u8])->Result<(),()>; fn read(&mut self, addr: u8, buf: &mut [u8])->Result<(),()>; }
