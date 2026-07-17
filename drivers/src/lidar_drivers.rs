//! LiDAR and Range Sensor Drivers
//! Velodyne, Sick, Livox, TeraRanger, etc.

use vortex_types::VortexResult;

#[derive(Debug, Clone, Copy)]
pub struct RangeSensorData {
    pub distance_mm: u16,
    pub signal_strength: u8,
    pub timestamp_ns: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct LidarPoint {
    pub x: f32, pub y: f32, pub z: f32,
    pub intensity: u8,
}

/// Velodyne VLP-16 (16-channel LiDAR)
pub struct VelodyneVlp16 {
    ethernet_port: u8,
    points: [LidarPoint; 30000],
    point_count: usize,
}

impl VelodyneVlp16 {
    pub fn new(ethernet_port: u8) -> Self {
        Self {
            ethernet_port,
            points: [LidarPoint { x: 0.0, y: 0.0, z: 0.0, intensity: 0 }; 30000],
            point_count: 0,
        }
    }

    pub fn init(&mut self) -> VortexResult<()> {
        Ok(())
    }

    pub fn get_point_cloud(&self) -> &[LidarPoint] {
        &self.points[..self.point_count]
    }
}

/// Sick S300 (2D LiDAR, for autonomous cars)
pub struct SickS300 {
    uart_port: u8,
    initialized: bool,
}

impl SickS300 {
    pub fn new(uart_port: u8) -> Self {
        Self {
            uart_port,
            initialized: false,
        }
    }

    pub fn init(&mut self) -> VortexResult<()> {
        self.initialized = true;
        Ok(())
    }

    pub fn read_scan(&mut self) -> VortexResult<[RangeSensorData; 541]> {
        Ok([RangeSensorData {
            distance_mm: 5000,
            signal_strength: 100,
            timestamp_ns: 0,
        }; 541])
    }
}

/// Livox Mid-360 (Compact 3D LiDAR)
pub struct LivoxMid360 {
    spi_port: u8,
    initialized: bool,
}

impl LivoxMid360 {
    pub fn new(spi_port: u8) -> Self {
        Self {
            spi_port,
            initialized: false,
        }
    }

    pub fn init(&mut self) -> VortexResult<()> {
        self.initialized = true;
        Ok(())
    }

    pub fn get_cloud(&self) -> VortexResult<Vec<LidarPoint>> {
        Ok(Vec::new())
    }
}

/// VL53L0X (Time-of-flight range sensor)
pub struct Vl53l0x {
    i2c_addr: u8,
    initialized: bool,
}

impl Vl53l0x {
    pub fn new(i2c_addr: u8) -> Self {
        Self {
            i2c_addr,
            initialized: false,
        }
    }

    pub fn init(&mut self) -> VortexResult<()> {
        self.initialized = true;
        Ok(())
    }

    pub fn read(&mut self) -> VortexResult<RangeSensorData> {
        Ok(RangeSensorData {
            distance_mm: 500,
            signal_strength: 100,
            timestamp_ns: 0,
        })
    }
}
