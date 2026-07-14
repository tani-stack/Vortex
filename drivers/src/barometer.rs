pub struct BaroData { pub pressure_pa: f32, pub alt_m: f32 } pub trait Barometer { fn read(&mut self)->BaroData; }
