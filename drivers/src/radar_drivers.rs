//! Radar Drivers
//! automotive radar, motion detection, etc.

use aero_types::AeroResult;

#[derive(Debug, Clone, Copy)]
pub struct RadarTarget {
    pub range: f32,           // meters
    pub velocity: f32,        // m/s
    pub angle: f32,           // degrees
    pub amplitude: f32,       // signal strength
    pub track_id: u16,
}

/// Texas Instruments IWR6843 - 60GHz Radar
pub struct IWR6843 {
    uart_port: u8,
    initialized: bool,
}

impl IWR6843 {
    pub fn new(uart_port: u8) -> Self {
        Self {
            uart_port,
            initialized: false,
        }
    }

    pub fn init(&mut self) -> AeroResult<()> {
        self.initialized = true;
        Ok(())
    }

    pub fn get_targets(&self) -> AeroResult<Vec<RadarTarget>> {
        Ok(Vec::new())
    }
}

/// Continental ARS408 - Automotive Radar
pub struct ContinentalArs408 {
    can_bus: u8,
    initialized: bool,
}

impl ContinentalArs408 {
    pub fn new(can_bus: u8) -> Self {
        Self {
            can_bus,
            initialized: false,
        }
    }

    pub fn init(&mut self) -> AeroResult<()> {
        self.initialized = true;
        Ok(())
    }

    pub fn detect_objects(&self) -> AeroResult<Vec<RadarTarget>> {
        Ok(Vec::new())
    }
}
