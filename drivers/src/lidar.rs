pub struct LidarPoint { pub distance_mm: u32, pub angle_mdeg: i32, pub intensity: u8 }
pub struct LidarStream { pub ring: [LidarPoint; 2048], pub head: usize }
impl LidarStream { pub fn push(&mut self, p: LidarPoint) { self.ring[self.head%2048]=p; self.head+=1; } }
