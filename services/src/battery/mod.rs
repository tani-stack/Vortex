pub mod fuel_gauge; pub struct BatteryState { pub voltage_mv: u32, pub current_ma: i32, pub soc_pct: u8 } impl BatteryState { pub fn is_critical(&self)->bool { self.soc_pct < 15 } }
