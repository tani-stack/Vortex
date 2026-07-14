pub trait UartBus { fn write_byte(&mut self, b: u8); fn read_byte(&mut self)->Option<u8>; }
