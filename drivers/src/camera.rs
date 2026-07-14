pub struct CameraFrame { pub id: u64, pub data: [u8; 0] } pub trait Camera { fn capture(&mut self)->Option<u64>; }
