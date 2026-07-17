//! Wheel Encoder Drivers
//! Odometry for robots and cars

use aero_types::AeroResult;

/// Rotary Encoder (incremental)
pub struct RotaryEncoder {
    clk_pin: u8,
    dt_pin: u8,
    position: i32,
}

impl RotaryEncoder {
    pub fn new(clk: u8, dt: u8) -> Self {
        Self {
            clk_pin: clk,
            dt_pin: dt,
            position: 0,
        }
    }

    pub fn init(&mut self) -> AeroResult<()> {
        Ok(())
    }

    pub fn get_position(&self) -> i32 {
        self.position
    }

    pub fn reset(&mut self) {
        self.position = 0;
    }
}

/// Wheel Speed Sensor (for cars and robots)
pub struct WheelSpeedSensor {
    gpio_pin: u8,
    ticks_per_revolution: u16,
    wheel_diameter_mm: f32,
}

impl WheelSpeedSensor {
    pub fn new(gpio_pin: u8, ticks_per_rev: u16, wheel_diameter: f32) -> Self {
        Self {
            gpio_pin,
            ticks_per_revolution: ticks_per_rev,
            wheel_diameter_mm: wheel_diameter,
        }
    }

    pub fn init(&mut self) -> AeroResult<()> {
        Ok(())
    }

    pub fn read_speed(&self) -> AeroResult<f32> {
        Ok(0.0)  // m/s
    }

    pub fn read_distance(&self) -> AeroResult<f32> {
        Ok(0.0)  // meters
    }
}
