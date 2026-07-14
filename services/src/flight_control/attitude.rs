use aero_types::Quaternion;
pub struct Pid { kp: f32, ki: f32, kd: f32, integral: f32 }
impl Pid { pub fn new(kp:f32,ki:f32,kd:f32)->Self{Self{kp,ki,kd,integral:0.0}} pub fn update(&mut self, err:f32, dt:f32)->f32 { self.integral+=err*dt; self.kp*err + self.ki*self.integral } }
pub struct AttitudeController { roll: Pid, pitch: Pid, yaw: Pid }
impl AttitudeController { pub fn run(&mut self, target: Quaternion, current: Quaternion) -> [f32;4] { [0.0;4] } }
