//! Environmental & Environmental Sensors
//! Air quality, humidity, gas sensors, etc.

use aero_types::AeroResult;

/// BME688 - 4-in-1 Environmental Sensor (Enhanced)
pub struct Bme688Enhanced {
    i2c_addr: u8,
    initialized: bool,
}

impl Bme688Enhanced {
    pub fn new(i2c_addr: u8) -> Self {
        Self {
            i2c_addr,
            initialized: false,
        }
    }

    pub fn init(&mut self) -> AeroResult<()> {
        self.initialized = true;
        Ok(())
    }

    pub fn read_all(&self) -> AeroResult<(f32, f32, f32, u16)> {
        // (temperature, humidity, pressure, gas_resistance)
        Ok((25.0, 50.0, 101325.0, 10000))
    }
}

/// CCS811 - VOC Air Quality Sensor
pub struct Ccs811 {
    i2c_addr: u8,
    initialized: bool,
}

impl Ccs811 {
    pub fn new(i2c_addr: u8) -> Self {
        Self {
            i2c_addr,
            initialized: false,
        }
    }

    pub fn init(&mut self) -> AeroResult<()> {
        self.initialized = true;
        Ok(())
    }

    pub fn read_voc(&self) -> AeroResult<u16> {
        Ok(400)  // eCO2 in ppm
    }
}

/// MQ135 - Air Quality Sensor (Analog)
pub struct Mq135 {
    adc_pin: u8,
    initialized: bool,
}

impl Mq135 {
    pub fn new(adc_pin: u8) -> Self {
        Self {
            adc_pin,
            initialized: false,
        }
    }

    pub fn init(&mut self) -> AeroResult<()> {
        self.initialized = true;
        Ok(())
    }

    pub fn read_ppm(&self) -> AeroResult<f32> {
        Ok(400.0)
    }
}

/// SCD30 - CO2 Sensor (NDIR)
pub struct Scd30 {
    i2c_addr: u8,
    initialized: bool,
}

impl Scd30 {
    pub fn new(i2c_addr: u8) -> Self {
        Self {
            i2c_addr,
            initialized: false,
        }
    }

    pub fn init(&mut self) -> AeroResult<()> {
        self.initialized = true;
        Ok(())
    }

    pub fn read(&self) -> AeroResult<(f32, f32, f32)> {
        // (co2_ppm, temperature, humidity)
        Ok((400.0, 25.0, 50.0))
    }
}
