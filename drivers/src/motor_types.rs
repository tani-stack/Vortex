#![no_std]
use core::fmt::Debug;

/// Generic Motor Error
#[derive(Debug, Clone, Copy)]
pub enum MotorError {
    NotArmed,
    InvalidCommand,
    BusFault,
    Overload,
    Overtemp,
}

/// DC Motor for cars and robots (PWM-based)
#[derive(Clone, Copy, Debug)]
pub struct DcMotorCommand {
    pub speed: i16, // -1000 to 1000 (reverse to forward)
    pub brake: bool,
}

pub trait DcMotor: Send + Sync {
    fn init(&mut self) -> Result<(), MotorError>;
    fn set_speed(&mut self, cmd: DcMotorCommand) -> Result<(), MotorError>;
    fn is_healthy(&self) -> bool;
}

/// Servo Motor for steering, robot joints (angle-based)
#[derive(Clone, Copy, Debug)]
pub struct ServoCommand {
    pub angle_deg: f32, // 0 to 180 degrees
}

pub trait ServoMotor: Send + Sync {
    fn init(&mut self) -> Result<(), MotorError>;
    fn set_angle(&mut self, cmd: ServoCommand) -> Result<(), MotorError>;
    fn get_position(&self) -> f32;
    fn is_healthy(&self) -> bool;
}

/// Stepper Motor for precise robot control
#[derive(Clone, Copy, Debug)]
pub struct StepperCommand {
    pub steps: i32,
    pub direction: bool, // true = forward, false = backward
    pub speed: u16, // steps per second
}

pub trait StepperMotor: Send + Sync {
    fn init(&mut self) -> Result<(), MotorError>;
    fn step(&mut self, cmd: StepperCommand) -> Result<(), MotorError>;
    fn is_healthy(&self) -> bool;
}

/// Brushless Motor (already exists, keeping for drones)
#[derive(Clone, Copy, Debug)]
pub struct BrushlessCommand {
    pub rpm: u32,
}

pub trait BrushlessMotor: Send + Sync {
    fn init(&mut self) -> Result<(), MotorError>;
    fn set_rpm(&mut self, cmd: BrushlessCommand) -> Result<(), MotorError>;
    fn is_healthy(&self) -> bool;
}

/// Unified Motor Trait
pub enum MotorType {
    Dc,
    Servo,
    Stepper,
    Brushless,
}

pub trait UnifiedMotor: Send + Sync {
    fn motor_type(&self) -> MotorType;
    fn init(&mut self) -> Result<(), MotorError>;
    fn is_healthy(&self) -> bool;
}
