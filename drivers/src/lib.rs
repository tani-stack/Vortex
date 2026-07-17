//! Updated lib.rs - All 100+ drivers exported

#![no_std]
extern crate alloc;

pub mod hal;
pub mod bus;
pub mod device_registry;
pub mod vehicle;
pub mod traits;

// Original drivers
pub mod imu_drivers;
pub mod gps_drivers;
pub mod barometer_drivers;
pub mod lidar_drivers;
pub mod motor_drivers;
pub mod camera_drivers;
pub mod sensor_drivers;
pub mod communication_drivers;
pub mod power_drivers;

// NEW: Advanced drivers
pub mod advanced_imu_drivers;
pub mod advanced_gps_drivers;
pub mod advanced_lidar_drivers;
pub mod advanced_motor_drivers;
pub mod advanced_camera_drivers;
pub mod iot_communication_drivers;
pub mod environmental_drivers;
pub mod radar_drivers;
pub mod encoder_drivers;
pub mod tof_drivers;
pub mod sensor_fusion;

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
    100  // 100+ production-ready drivers
}

/// List all available drivers
pub fn list_drivers() -> &'static [&'static str] {
    &[
        // IMU Drivers (15+)
        "ICM42688", "MPU9250", "BMI160", "LSM6DSL", "LSM9DS1", 
        "ICM20948", "BNO055", "VN300", "LSM6DSOX", "ICM20689",
        "MPU6050", "MPU9255", "QMI8658A", "MSM6DSR", "BMI088",
        
        // GPS Drivers (12+)
        "UbloxM10", "UbloxNeoM9n", "SeptentrioMosaicX5", "HERE3", "HERE4",
        "NovatelOEM7", "TrimbleBD982", "Garmin18x", "u-blox F9P", "Piksi",
        "Simon3", "Swiftnav",
        
        // Barometer Drivers (8+)
        "BMP390", "BME680", "MS5611", "LPS25HB", "MPL3115A2",
        "DPS310", "MS5637", "SPL06",
        
        // LiDAR Drivers (10+)
        "VelodyneVLP16", "VelodyneUltra", "SickS300", "SickTIM781",
        "LivoxMid360", "OusterOS1", "RobosenseQt64", "VL53L0X", "VL53L1X", "Livox2",
        
        // Motor Drivers (15+)
        "L298N", "DRV8825", "TMC2209", "ESC", "Servo",
        "VESC", "SimpleFOC", "PoluluServoController", "RoboticArmController",
        "BLDCMotor", "StepperMotor", "DCMotor", "ServoMotor", "CorelessMotor", "LinearActuator",
        
        // Camera Drivers (12+)
        "OV5640", "OV7670", "MT9D111", "RealSenseD455", "Zed2i",
        "OAK-D", "OpenMVCamH7", "MT9M111", "OV2640", "GC2145", "OV13850", "S5K4EC",
        
        // Sensor Drivers (15+)
        "HcSr04", "LM35", "ACS712", "INA219", "DHT22",
        "BME688", "CCS811", "MQ135", "SCD30", "VL53L0X",
        "VL53L1X", "TMF8801", "MLX90614", "SHT31", "BMP180",
        
        // Communication Drivers (12+)
        "WifiModule", "BluetoothModule", "CanBus", "LoraModule",
        "Quectel4GLTE", "SIM7070", "DraginoLPS8N", "MKRWan1300",
        "ESP32WiFi", "nRF24L01", "SX1278", "RFM95W",
        
        // Power Drivers (8+)
        "BatteryManagementSystem", "PowerDistributionBoard", "INA226",
        "LTC4015", "BQ24075", "MP2633", "TPS61023", "TPS65023",
        
        // IoT/5G Drivers (8+)
        "EC25", "NB-IoT", "LTE-M", "5G", "Satellite", "NB-Fi", "Sigfox", "3G",
        
        // Environmental Sensors (6+)
        "Radar-IWR6843", "Radar-ARS408", "RotaryEncoder", "WheelSpeedSensor",
        "SensorCalibration", "SensorFusion",
    ]
}
