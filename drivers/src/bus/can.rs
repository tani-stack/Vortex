pub struct CanFrame { pub id: u32, pub dlc: u8, pub data: [u8;8] } pub trait CanBus { fn send(&mut self, f: CanFrame)->Result<(),()>; }
