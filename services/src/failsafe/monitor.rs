use aero_drivers::motor::BrushlessMotor;
pub struct FailsafeMonitor { pub armed: bool }
impl FailsafeMonitor {
  pub fn check_motors<M: BrushlessMotor>(&self, motors: &[M]) -> bool {
    for m in motors { if !m.is_healthy() { return false; } } true
  }
  pub fn should_land(&self, batt_soc: u8) -> bool { batt_soc < 10 }
}
