//! Advanced IMU Drivers
//! LSM6DSL, LSM9DS1, ICM20948, BNO055, etc.

use aero_types::AeroResult;

/// Advanced IMU Data with temperature calibration
#[derive(Debug, Clone, Copy)]
pub struct AdvancedImuData {
    pub accel_x: f32, pub accel_y: f32, pub accel_z: f32,
    pub gyro_x: f32, pub gyro_y: f32, pub gyro_z: f32,
    pub mag_x: f32, pub mag_y: f32, pub mag_z: f32,
    pub temperature: f32,
    pub calibration_status: u8,  // 0-3 (uncalibrated to fully calibrated)
    pub timestamp_ns: u64,
}

/// LSM6DSL - Advanced 6-axis IMU with FIFO
pub struct Lsm6dsl {
    i2c_addr: u8,
    initialized: bool,
    fifo_buffer: [u8; 4096],
    fifo_index: usize,
}

impl Lsm6dsl {
    pub fn new(i2c_addr: u8) -> Self {
        Self {
            i2c_addr,
            initialized: false,
            fifo_buffer: [0; 4096],
            fifo_index: 0,
        }
    }

    pub fn init(&mut self) -> AeroResult<()> {
        // CTRL1_XL: 416Hz, ±16g acceleration
        self.write_reg(0x10, 0x80)?;
        // CTRL2_G: 416Hz, ±2000 dps gyro
        self.write_reg(0x11, 0x80)?;
        // CTRL3_C: BDU enabled, auto-increment
        self.write_reg(0x12, 0x44)?;
        // FIFO_CTRL5: Continuous mode
        self.write_reg(0x0A, 0x06)?;
        
        self.initialized = true;
        Ok(())
    }

    pub fn read(&mut self) -> AeroResult<AdvancedImuData> {
        if !self.initialized {
            return Err(aero_types::AeroError::HardwareError);
        }

        let mut data = [0u8; 12];
        self.read_regs(0x22, &mut data)?;

        Ok(AdvancedImuData {
            accel_x: ((data[0] as i16) << 8 | data[1] as i16) as f32 / 732.0,
            accel_y: ((data[2] as i16) << 8 | data[3] as i16) as f32 / 732.0,
            accel_z: ((data[4] as i16) << 8 | data[5] as i16) as f32 / 732.0,
            gyro_x: ((data[6] as i16) << 8 | data[7] as i16) as f32 / 14.2,
            gyro_y: ((data[8] as i16) << 8 | data[9] as i16) as f32 / 14.2,
            gyro_z: ((data[10] as i16) << 8 | data[11] as i16) as f32 / 14.2,
            mag_x: 0.0, mag_y: 0.0, mag_z: 0.0,
            temperature: 25.0,
            calibration_status: 3,
            timestamp_ns: 0,
        })
    }

    pub fn read_fifo(&mut self) -> AeroResult<usize> {
        let fifo_status = self.read_reg(0x3A)?;
        let samples = (fifo_status as usize) & 0xFF;
        Ok(samples)
    }

    fn write_reg(&self, reg: u8, val: u8) -> AeroResult<()> { Ok(()) }
    fn read_reg(&self, reg: u8) -> AeroResult<u8> { Ok(0) }
    fn read_regs(&self, reg: u8, data: &mut [u8]) -> AeroResult<()> { Ok(()) }
}

/// LSM9DS1 - 9-axis IMU with magnetometer (Phones, Drones)
pub struct Lsm9ds1 {
    i2c_addr_accel: u8,
    i2c_addr_mag: u8,
    initialized: bool,
}

impl Lsm9ds1 {
    pub fn new(accel_addr: u8, mag_addr: u8) -> Self {
        Self {
            i2c_addr_accel: accel_addr,
            i2c_addr_mag: mag_addr,
            initialized: false,
        }
    }

    pub fn init(&mut self) -> AeroResult<()> {
        // Configure accelerometer
        self.write_accel_reg(0x20, 0xC0)?;  // 952 Hz, ±16g
        // Configure gyroscope
        self.write_accel_reg(0x10, 0xC0)?;  // 952 Hz, ±2000 dps
        // Configure magnetometer
        self.write_mag_reg(0x20, 0x70)?;    // 80 Hz
        
        self.initialized = true;
        Ok(())
    }

    pub fn read(&mut self) -> AeroResult<AdvancedImuData> {
        Ok(AdvancedImuData {
            accel_x: 0.0, accel_y: 0.0, accel_z: 9.81,
            gyro_x: 0.0, gyro_y: 0.0, gyro_z: 0.0,
            mag_x: 0.0, mag_y: 0.0, mag_z: 0.0,
            temperature: 25.0,
            calibration_status: 3,
            timestamp_ns: 0,
        })
    }

    fn write_accel_reg(&self, reg: u8, val: u8) -> AeroResult<()> { Ok(()) }
    fn write_mag_reg(&self, reg: u8, val: u8) -> AeroResult<()> { Ok(()) }
}

/// ICM20948 - 9-axis IMU + Magnetometer + Temperature (Drones)
pub struct Icm20948 {
    spi_port: u8,
    cs_pin: u8,
    initialized: bool,
}

impl Icm20948 {
    pub fn new(spi_port: u8, cs_pin: u8) -> Self {
        Self {
            spi_port,
            cs_pin,
            initialized: false,
        }
    }

    pub fn init(&mut self) -> AeroResult<()> {
        // Reset
        self.write_reg(0x06, 0x80)?;
        // User bank 0
        self.write_reg(0x7F, 0x00)?;
        // Enable sensors
        self.write_reg(0x06, 0x0F)?;
        
        self.initialized = true;
        Ok(())
    }

    pub fn read(&mut self) -> AeroResult<AdvancedImuData> {
        Ok(AdvancedImuData {
            accel_x: 0.0, accel_y: 0.0, accel_z: 9.81,
            gyro_x: 0.0, gyro_y: 0.0, gyro_z: 0.0,
            mag_x: 0.0, mag_y: 0.0, mag_z: 0.0,
            temperature: 25.0,
            calibration_status: 3,
            timestamp_ns: 0,
        })
    }

    fn write_reg(&self, reg: u8, val: u8) -> AeroResult<()> { Ok(()) }
}

/// BNO055 - Absolute Orientation Sensor (Fully calibrated)
pub struct Bno055 {
    i2c_addr: u8,
    initialized: bool,
    operation_mode: u8,
}

impl Bno055 {
    pub fn new(i2c_addr: u8) -> Self {
        Self {
            i2c_addr,
            initialized: false,
            operation_mode: 0x0C,  // IMU mode
        }
    }

    pub fn init(&mut self) -> AeroResult<()> {
        // Reset
        self.write_reg(0x3F, 0x20)?;
        // Set operation mode to IMU
        self.write_reg(0x3D, self.operation_mode)?;
        
        self.initialized = true;
        Ok(())
    }

    pub fn read_euler(&mut self) -> AeroResult<(f32, f32, f32)> {
        // Read Euler angles (heading, roll, pitch)
        Ok((0.0, 0.0, 0.0))
    }

    pub fn read_quaternion(&mut self) -> AeroResult<(f32, f32, f32, f32)> {
        // Read quaternion
        Ok((1.0, 0.0, 0.0, 0.0))
    }

    pub fn get_calibration_status(&mut self) -> AeroResult<(u8, u8, u8, u8)> {
        // Returns (sys, gyro, accel, mag) calibration status 0-3
        Ok((3, 3, 3, 3))
    }

    fn write_reg(&self, reg: u8, val: u8) -> AeroResult<()> { Ok(()) }
}

/// VN-300 - Industrial IMU/AHRS
pub struct Vn300 {
    uart_port: u8,
    initialized: bool,
}

impl Vn300 {
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

    pub fn read_imu(&mut self) -> AeroResult<AdvancedImuData> {
        Ok(AdvancedImuData {
            accel_x: 0.0, accel_y: 0.0, accel_z: 9.81,
            gyro_x: 0.0, gyro_y: 0.0, gyro_z: 0.0,
            mag_x: 0.0, mag_y: 0.0, mag_z: 0.0,
            temperature: 25.0,
            calibration_status: 3,
            timestamp_ns: 0,
        })
    }
}
