use aero_types::{Milliamps, Millivolts, Celsius};
use bitflags::bitflags;
#[derive(Debug, Clone, Copy)] pub struct MotorTelemetry { pub rpm_actual: u32, pub current_ma: Milliamps, pub voltage_mv: Millivolts, pub temp_c: Celsius, pub fault: MotorFault, }
bitflags!{ #[derive(Debug, Clone, Copy, PartialEq, Eq)] pub struct MotorFault: u32 { const NONE=0; const STALL=1; const OVER_CURRENT=2; const OVER_TEMP=4; const DESYNC=8; const ESC_TIMEOUT=16; } }
#[derive(Debug)] pub enum MotorError { NotArmed, InvalidRpm, BusFault, FailsafeActive, HardwareFault(MotorFault), }
pub trait BrushlessMotor: Send+Sync { fn id(&self)->u8; fn arm(&mut self)->Result<(), MotorError>; fn disarm(&mut self)->Result<(), MotorError>; fn set_rpm(&mut self, rpm: u32)->Result<(), MotorError>; fn brake(&mut self)->Result<(), MotorError>; fn get_telemetry(&self)->MotorTelemetry; fn is_healthy(&self)->bool { self.get_telemetry().fault.is_empty() } }
pub struct DshotMotor<B> { pub bus: B, pub id: u8, pub cache: MotorTelemetry }
impl<B: crate::bus::spi::SpiBus> BrushlessMotor for DshotMotor<B> {
  fn id(&self)->u8{self.id} fn arm(&mut self)->Result<(),MotorError>{Ok(())} fn disarm(&mut self)->Result<(),MotorError>{Ok(())}
  fn set_rpm(&mut self, rpm: u32)->Result<(),MotorError>{ if rpm>25000 {return Err(MotorError::InvalidRpm);} Ok(()) }
  fn brake(&mut self)->Result<(),MotorError>{Ok(())} fn get_telemetry(&self)->MotorTelemetry{self.cache}
}
