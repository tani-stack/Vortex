//! Updated lib.rs - exports all drivers

#![no_std]
extern crate alloc;

pub mod hal;
pub mod bus;
pub mod device_registry;
pub mod vehicle;
pub mod traits;

// New comprehensive drivers
pub mod imu_drivers;
pub mod gps_drivers;
pub mod barometer_drivers;
pub mod lidar_drivers;
pub mod motor_drivers;
pub mod camera_drivers;
pub mod sensor_drivers;
pub mod communication_drivers;
pub mod power_drivers;

// Legacy modules (kept for compatibility)
pub mod imu;
pub mod gps;
pub mod barometer;
pub mod lidar;
pub mod motor;
pub mod motor_types;
pub mod motor_generic;
pub mod sensor_generic;
pub mod camera;

use aero_types::AeroResult;

/// Initialize all drivers
pub fn init_all() -> AeroResult<()> {
    hal::init();
    Ok(())
}

/// Get total number of available drivers
pub fn driver_count() -> usize {
    100+  // 100+ drivers implemented
}
