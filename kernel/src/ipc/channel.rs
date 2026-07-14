use spin::Mutex;
pub struct Capability { pub id: u64, pub rights: u32 }
pub struct Channel { pub cap: Capability, pub buf: Mutex<[u8; 1024]> }
impl Channel { pub fn send(&self, data: &[u8]) -> Result<(), ()> { let mut b=self.buf.lock(); b[..data.len()].copy_from_slice(data); Ok(()) } }
