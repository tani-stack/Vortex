#![no_std]

//! Core types for Vortex OS - Vehicle Control & Management

use core::fmt;

/// ============ SENSOR DATA TYPES ============
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Default> Default for Vector3<T> {
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
            z: T::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IMUData {
    pub accel: Vector3<f32>,      // m/s²
    pub gyro: Vector3<f32>,       // rad/s
    pub mag: Vector3<f32>,        // Gauss
    pub temperature: f32,         // °C
    pub timestamp_ms: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GPSData {
    pub latitude: f64,            // degrees
    pub longitude: f64,           // degrees
    pub altitude: f32,            // meters
    pub speed: f32,               // m/s
    pub heading: f32,             // degrees
    pub satellites: u8,
    pub hdop: f32,               // Horizontal DOP
    pub timestamp_ms: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BarometerData {
    pub altitude: f32,            // meters
    pub pressure: f32,            // Pa
    pub temperature: f32,         // °C
    pub timestamp_ms: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LidarData {
    pub distance: f32,            // meters
    pub intensity: u8,
    pub angle: f32,               // degrees (if multi-beam)
    pub timestamp_ms: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatteryData {
    pub voltage_mv: u32,
    pub current_ma: u32,
    pub capacity_mah: u32,
    pub percentage: u8,           // 0-100
    pub temperature: f32,         // °C
    pub cell_count: u8,
    pub timestamp_ms: u32,
}

/// ============ MOTOR COMMAND TYPES ============
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotorType {
    Brushless,
    Brushed,
    Stepper,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MotorCommand {
    pub motor_id: u8,
    pub command: MotorCommandType,
    pub timestamp_ms: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MotorCommandType {
    SetRPM(u16),
    SetThrottle(f32),        // 0.0 to 1.0
    SetTorque(f32),          // Nm
    Brake,
    Coast,
}

/// ============ VEHICLE STATE ============
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VehicleMode {
    Disarmed,
    Armed,
    InFlight,
    Landing,
    Emergency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlightMode {
    Stabilize,
    Altitude,
    Position,
    Auto,
    Manual,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VehicleState {
    pub mode: VehicleMode,
    pub flight_mode: FlightMode,
    pub position: Vector3<f32>,   // NED frame (meters)
    pub velocity: Vector3<f32>,   // m/s
    pub attitude: Vector3<f32>,   // roll, pitch, yaw (radians)
    pub battery_voltage_mv: u32,
    pub armed: bool,
    pub timestamp_ms: u32,
}

/// ============ VEHICLE CONFIGURATION ============
#[derive(Debug, Clone, Copy)]
pub struct VehicleConfig {
    pub vehicle_id: u8,
    pub vehicle_type: VehicleType,
    pub mass_kg: f32,
    pub max_speed: f32,           // m/s
    pub max_acceleration: f32,    // m/s²
    pub battery_capacity_mah: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VehicleType {
    Quadcopter,
    Hexacopter,
    Airplane,
    GroundRobot,
    Car,
    Boat,
}

/// ============ ERRORS ============
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VortexError {
    HardwareNotFound,
    CommunicationError,
    CalibrationError,
    SensorError,
    MotorError,
    BatteryError,
    MemoryError,
    Timeout,
    InvalidParameter,
    NotInitialized,
    AlreadyInitialized,
    Unknown,
}

impl fmt::Display for VortexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VortexError::HardwareNotFound => write!(f, "Hardware not found"),
            VortexError::CommunicationError => write!(f, "Communication error"),
            VortexError::CalibrationError => write!(f, "Calibration error"),
            VortexError::SensorError => write!(f, "Sensor error"),
            VortexError::MotorError => write!(f, "Motor error"),
            VortexError::BatteryError => write!(f, "Battery error"),
            VortexError::MemoryError => write!(f, "Memory error"),
            VortexError::Timeout => write!(f, "Timeout"),
            VortexError::InvalidParameter => write!(f, "Invalid parameter"),
            VortexError::NotInitialized => write!(f, "Not initialized"),
            VortexError::AlreadyInitialized => write!(f, "Already initialized"),
            VortexError::Unknown => write!(f, "Unknown error"),
        }
    }
}

/// Result type alias
pub type VortexResult<T> = Result<T, VortexError>;
