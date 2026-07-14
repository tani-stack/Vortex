pub struct GpsFix { pub lat: f64, pub lon: f64, pub alt: f32 } pub trait Gps { fn fix(&mut self)->Option<GpsFix>; }
