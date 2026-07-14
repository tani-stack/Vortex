pub struct Ekf { pub state: [f32; 16] } impl Ekf { pub fn predict(&mut self, dt:f32) {} pub fn update_gps(&mut self, lat:f64, lon:f64) {} }
