pub struct ZeroCopyRing { pub buf: [[u8; 256]; 64], pub head: usize, pub tail: usize } impl ZeroCopyRing { pub fn push(&mut self, d: [u8;256]) { self.buf[self.head%64]=d; self.head+=1; } }
