//! Universal Sensor Interface - Works with any sensor
//! IMU, GPS, Barometer, Magnetometer, etc.

use crate::device_registry::{GenericDevice, DeviceMetadata, DeviceId, DeviceStatus, HealthStatus};
use aero_types::AeroResult;
use core::fmt;

/// Unified sensor reading
#[derive(Debug, Clone, Copy)]
pub struct SensorReading {
    pub value: f32,
    pub unit: SensorUnit,
    pub timestamp_ns: u64,
    pub confidence: f32, // 0.0 to 1.0
}

/// Sensor units
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorUnit {
    Acceleration,      // m/s²
    AngularVelocity,   // rad/s
    Temperature,       // °C
    Pressure,          // Pa
    Magnetic,          // µT
    Distance,          // mm
    Voltage,           // mV
    Current,           // mA
    Angle,             // degrees
    Raw,
}

impl fmt::Display for SensorUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SensorUnit::Acceleration => write!(f, "m/s²"),
            SensorUnit::AngularVelocity => write!(f, "rad/s"),
            SensorUnit::Temperature => write!(f, "°C"),
            SensorUnit::Pressure => write!(f, "Pa"),
            SensorUnit::Magnetic => write!(f, "µT"),
            SensorUnit::Distance => write!(f, "mm"),
            SensorUnit::Voltage => write!(f, "mV"),
            SensorUnit::Current => write!(f, "mA"),
            SensorUnit::Angle => write!(f, "°"),
            SensorUnit::Raw => write!(f, "raw"),
        }
    }
}

/// Generic sensor trait
pub trait Sensor: GenericDevice {
    fn read_value(&mut self) -> AeroResult<SensorReading>;
    fn read_raw(&mut self) -> AeroResult<u16> {
        Ok(0)
    }
    fn calibrate(&mut self) -> AeroResult<()> {
        Ok(())
    }
    fn set_sample_rate(&mut self, hz: u32) -> AeroResult<()>;
    fn get_sample_rate(&self) -> u32 {
        self.metadata().sample_rate_hz.unwrap_or(100)
    }
    fn get_resolution(&self) -> u8 {
        12
    }
}

/// IMU Sensor Data (6-axis)
#[derive(Debug, Clone, Copy)]
pub struct ImuData {
    pub accel_x: f32,
    pub accel_y: f32,
    pub accel_z: f32,
    pub gyro_x: f32,
    pub gyro_y: f32,
    pub gyro_z: f32,
    pub temperature: f32,
    pub timestamp_ns: u64,
}

/// Generic IMU Driver
pub struct GenericImu {
    id: DeviceId,
    metadata: DeviceMetadata,
    initialized: bool,
    last_data: ImuData,
}

impl GenericImu {
    pub fn new(id: DeviceId, metadata: DeviceMetadata) -> Self {
        Self {
            id,
            metadata,
            initialized: false,
            last_data: ImuData {
                accel_x: 0.0, accel_y: 0.0, accel_z: 0.0,
                gyro_x: 0.0, gyro_y: 0.0, gyro_z: 0.0,
                temperature: 25.0,
                timestamp_ns: 0,
            },
        }
    }

    pub fn get_imu_data(&self) -> ImuData {
        self.last_data
    }
}

impl GenericDevice for GenericImu {
    fn id(&self) -> DeviceId { self.id }
    fn metadata(&self) -> &DeviceMetadata { &self.metadata }
    fn init(&mut self) -> AeroResult<()> { self.initialized = true; Ok(()) }
    fn deinit(&mut self) -> AeroResult<()> { self.initialized = false; Ok(()) }
    fn reset(&mut self) -> AeroResult<()> { Ok(()) }
    fn status(&self) -> DeviceStatus { if self.initialized { DeviceStatus::Ready } else { DeviceStatus::Uninitialized } }
    fn health(&self) -> HealthStatus { if self.initialized { HealthStatus::Healthy } else { HealthStatus::Offline } }
    fn read_property(&self, _key: &str) -> AeroResult<u32> { Ok(0) }
    fn write_property(&mut self, _key: &str, _value: u32) -> AeroResult<()> { Ok(()) }
    fn read_raw(&mut self, _buf: &mut [u8]) -> AeroResult<usize> { Ok(0) }
    fn write_raw(&mut self, _data: &[u8]) -> AeroResult<()> { Ok(()) }
}

impl Sensor for GenericImu {
    fn read_value(&mut self) -> AeroResult<SensorReading> {
        let accel_magnitude = (self.last_data.accel_x.powi(2) + self.last_data.accel_y.powi(2) + self.last_data.accel_z.powi(2)).sqrt();
        Ok(SensorReading {
            value: accel_magnitude,
            unit: SensorUnit::Acceleration,
            timestamp_ns: self.last_data.timestamp_ns,
            confidence: 0.95,
        })
    }
    fn set_sample_rate(&mut self, hz: u32) -> AeroResult<()> {
        Ok(())
    }
}

/// GPS Sensor Data
#[derive(Debug, Clone, Copy)]
pub struct GpsData {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f32,
    pub satellites: u8,
    pub fix_type: u8,
    pub hdop: f32,
    pub timestamp_ns: u64,
}

/// Generic GPS Driver
pub struct GenericGps {
    id: DeviceId,
    metadata: DeviceMetadata,
    initialized: bool,
    last_data: GpsData,
}

impl GenericGps {
    pub fn new(id: DeviceId, metadata: DeviceMetadata) -> Self {
        Self {
            id,
            metadata,
            initialized: false,
            last_data: GpsData {
                latitude: 0.0, longitude: 0.0, altitude: 0.0,
                satellites: 0, fix_type: 0, hdop: 999.0, timestamp_ns: 0,
            },
        }
    }

    pub fn get_gps_data(&self) -> GpsData {
        self.last_data
    }
}

impl GenericDevice for GenericGps {
    fn id(&self) -> DeviceId { self.id }
    fn metadata(&self) -> &DeviceMetadata { &self.metadata }
    fn init(&mut self) -> AeroResult<()> { self.initialized = true; Ok(()) }
    fn deinit(&mut self) -> AeroResult<()> { self.initialized = false; Ok(()) }
    fn reset(&mut self) -> AeroResult<()> { Ok(()) }
    fn status(&self) -> DeviceStatus { if self.initialized { DeviceStatus::Ready } else { DeviceStatus::Uninitialized } }
    fn health(&self) -> HealthStatus { if self.last_data.fix_type >= 3 { HealthStatus::Healthy } else { HealthStatus::Warning } }
    fn read_property(&self, _key: &str) -> AeroResult<u32> { Ok(0) }
    fn write_property(&mut self, _key: &str, _value: u32) -> AeroResult<()> { Ok(()) }
    fn read_raw(&mut self, _buf: &mut [u8]) -> AeroResult<usize> { Ok(0) }
    fn write_raw(&mut self, _data: &[u8]) -> AeroResult<()> { Ok(()) }
}

impl Sensor for GenericGps {
    fn read_value(&mut self) -> AeroResult<SensorReading> {
        Ok(SensorReading {
            value: self.last_data.altitude,
            unit: SensorUnit::Distance,
            timestamp_ns: self.last_data.timestamp_ns,
            confidence: if self.last_data.fix_type >= 3 { 0.95 } else { 0.1 },
        })
    }
    fn set_sample_rate(&mut self, _hz: u32) -> AeroResult<()> { Ok(()) }
}

/// Barometer Sensor Data
#[derive(Debug, Clone, Copy)]
pub struct BarometerData {
    pub pressure: f32,
    pub temperature: f32,
    pub altitude: f32,
    pub timestamp_ns: u64,
}

/// Generic Barometer Driver
pub struct GenericBarometer {
    id: DeviceId,
    metadata: DeviceMetadata,
    initialized: bool,
    last_data: BarometerData,
}

impl GenericBarometer {
    pub fn new(id: DeviceId, metadata: DeviceMetadata) -> Self {
        Self {
            id,
            metadata,
            initialized: false,
            last_data: BarometerData {
                pressure: 101325.0,
                temperature: 25.0,
                altitude: 0.0,
                timestamp_ns: 0,
            },
        }
    }

    pub fn get_barometer_data(&self) -> BarometerData {
        self.last_data
    }
}

impl GenericDevice for GenericBarometer {
    fn id(&self) -> DeviceId { self.id }
    fn metadata(&self) -> &DeviceMetadata { &self.metadata }
    fn init(&mut self) -> AeroResult<()> { self.initialized = true; Ok(()) }
    fn deinit(&mut self) -> AeroResult<()> { self.initialized = false; Ok(()) }
    fn reset(&mut self) -> AeroResult<()> { Ok(()) }
    fn status(&self) -> DeviceStatus { if self.initialized { DeviceStatus::Ready } else { DeviceStatus::Uninitialized } }
    fn health(&self) -> HealthStatus { HealthStatus::Healthy }
    fn read_property(&self, _key: &str) -> AeroResult<u32> { Ok(0) }
    fn write_property(&mut self, _key: &str, _value: u32) -> AeroResult<()> { Ok(()) }
    fn read_raw(&mut self, _buf: &mut [u8]) -> AeroResult<usize> { Ok(0) }
    fn write_raw(&mut self, _data: &[u8]) -> AeroResult<()> { Ok(()) }
}

impl Sensor for GenericBarometer {
    fn read_value(&mut self) -> AeroResult<SensorReading> {
        Ok(SensorReading {
            value: self.last_data.pressure,
            unit: SensorUnit::Pressure,
            timestamp_ns: self.last_data.timestamp_ns,
            confidence: 0.98,
        })
    }
    fn set_sample_rate(&mut self, _hz: u32) -> AeroResult<()> { Ok(()) }
}
// Universal sensor abstraction layer
