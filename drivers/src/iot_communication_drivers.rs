//! 5G/IoT Communication Drivers
//! 4G LTE, NB-IoT, LoRaWAN modules

use aero_types::AeroResult;

/// Quectel EC25 - 4G LTE modem
pub struct Quectel4gLte {
    uart_port: u8,
    initialized: bool,
    signal_strength: u8,
}

impl Quectel4gLte {
    pub fn new(uart_port: u8) -> Self {
        Self {
            uart_port,
            initialized: false,
            signal_strength: 0,
        }
    }

    pub fn init(&mut self) -> AeroResult<()> {
        // AT+CFUN=1
        self.send_at_cmd("AT+CFUN=1")?;
        self.initialized = true;
        Ok(())
    }

    pub fn connect(&mut self, apn: &str) -> AeroResult<()> {
        // AT+CGACT=1,1
        self.send_at_cmd("AT+CGACT=1,1")?;
        Ok(())
    }

    pub fn send_http_post(&self, url: &str, data: &[u8]) -> AeroResult<()> {
        Ok(())
    }

    pub fn signal_strength(&mut self) -> AeroResult<u8> {
        Ok(self.signal_strength)
    }

    fn send_at_cmd(&self, cmd: &str) -> AeroResult<()> { Ok(()) }
}

/// SIM7070 - NB-IoT/LTE-M modem
pub struct Sim7070 {
    uart_port: u8,
    initialized: bool,
}

impl Sim7070 {
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

    pub fn mqtt_connect(&mut self, broker: &str, port: u16) -> AeroResult<()> {
        Ok(())
    }

    pub fn mqtt_publish(&self, topic: &str, payload: &[u8]) -> AeroResult<()> {
        Ok(())
    }
}

/// Dragino LPS8N - LoRaWAN Gateway
pub struct DraginoLps8n {
    spi_port: u8,
    reset_pin: u8,
}

impl DraginoLps8n {
    pub fn new(spi_port: u8, reset_pin: u8) -> Self {
        Self {
            spi_port,
            reset_pin,
        }
    }

    pub fn init(&mut self) -> AeroResult<()> {
        Ok(())
    }

    pub fn send_lora(&self, data: &[u8], spreading_factor: u8) -> AeroResult<()> {
        Ok(())
    }
}

/// MKR WAN 1300 - Arduino LoRaWAN board
pub struct MkrWan1300 {
    uart_port: u8,
}

impl MkrWan1300 {
    pub fn new(uart_port: u8) -> Self {
        Self { uart_port }
    }

    pub fn init(&mut self) -> AeroResult<()> {
        Ok(())
    }

    pub fn send_downlink(&self, port: u8, data: &[u8]) -> AeroResult<()> {
        Ok(())
    }
}
