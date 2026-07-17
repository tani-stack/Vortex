use vortex_drivers::motor::BrushlessMotor;
static SHOULD_LAND: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
pub fn request_land() {
    SHOULD_LAND.store(true, core::sync::atomic::Ordering::SeqCst);
}
pub struct FailsafeMonitor {
    pub armed: bool,
}
impl FailsafeMonitor {
    pub fn new() -> Self {
        Self { armed: false }
    }
    pub fn check_motors<M: BrushlessMotor>(&self, ms: &[M]) -> bool {
        for m in ms {
            if !m.is_healthy() {
                return false;
            }
        }
        true
    }
    pub fn should_land(&self, soc: u8) -> bool {
        soc < 10 || SHOULD_LAND.load(core::sync::atomic::Ordering::Relaxed)
    }
}
