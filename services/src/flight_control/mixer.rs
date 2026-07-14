pub enum Frame { QuadX, HexaX }
pub struct MotorMixer { frame: Frame }
impl MotorMixer {
  pub fn new(f: Frame)->Self{Self{frame:f}}
  pub fn mix(&self, thrust:f32, roll:f32, pitch:f32, yaw:f32)->[u32;8] {
    match self.frame { Frame::QuadX => { let m=[thrust+roll+pitch-yaw, thrust-roll+pitch+yaw, thrust-roll-pitch-yaw, thrust+roll-pitch+yaw]; [m[0] as u32, m[1] as u32, m[2] as u32, m[3] as u32,0,0,0,0] } _=> [0;8] }
  }
}
