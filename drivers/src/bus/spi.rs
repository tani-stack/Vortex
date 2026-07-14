pub trait SpiBus { fn transfer(&mut self, tx: &[u8], rx: &mut [u8])->Result<(),()>; fn write(&mut self, d: &[u8])->Result<(),()>{ self.transfer(d, &mut []) } }
