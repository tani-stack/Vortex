//! I2C (Inter-Integrated Circuit) Master abstraction

use vortex_types::VortexResult;
use super::mmio;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I2cAddress(pub u8);

impl I2cAddress {
    pub fn new(addr: u8) -> Self {
        I2cAddress(addr)
    }
}

/// I2C Master trait
pub trait I2cMaster: Send + Sync {
    fn write(&mut self, addr: I2cAddress, data: &[u8]) -> VortexResult<()>;
    fn read(&mut self, addr: I2cAddress, reg: u8, buffer: &mut [u8]) -> VortexResult<()>;
    fn write_read(&mut self, addr: I2cAddress, write_data: &[u8], read_buffer: &mut [u8]) -> VortexResult<()>;
}

/// ARM64 I2C1 implementation (Raspberry Pi / QEMU style)
pub struct Arm64I2c1 {
    baudrate: u32,
    initialized: bool,
}

impl Arm64I2c1 {
    pub fn new() -> Self {
        Self {
            baudrate: 100_000,  // 100 kHz
            initialized: false,
        }
    }

    pub fn init(&mut self) -> VortexResult<()> {
        // Configure I2C clock dividers
        let clock_rate = 250_000_000;  // 250 MHz assuming
        let div = clock_rate / (16 * self.baudrate);
        
        // BSC1_DIV register
        mmio::mmio_write(0x0804_0000, div & 0xFFFF);
        
        // Enable I2C
        mmio::mmio_write(0x0804_0004, 0x01);
        
        self.initialized = true;
        Ok(())
    }
}

impl I2cMaster for Arm64I2c1 {
    fn write(&mut self, addr: I2cAddress, data: &[u8]) -> VortexResult<()> {
        if !self.initialized {
            return Err(vortex_types::VortexError::HardwareError);
        }

        // Set slave address
        mmio::mmio_write(0x0804_0008, (addr.0 as u32) << 1);  // Shift for R/W bit
        
        // Write data to FIFO
        for &byte in data {
            mmio::mmio_write(0x0804_0040, byte as u32);
            
            // Wait for byte to be transmitted
            loop {
                let status = mmio::mmio_read(0x0804_0004);
                if (status & 0x02) != 0 {  // TA bit (Transfer Active)
                    break;
                }
            }
        }
        
        // Wait for transmission complete
        loop {
            let status = mmio::mmio_read(0x0804_0004);
            if (status & 0x02) == 0 && (status & 0x40) != 0 {  // Done bit
                break;
            }
        }
        
        Ok(())
    }

    fn read(&mut self, addr: I2cAddress, reg: u8, buffer: &mut [u8]) -> VortexResult<()> {
        if !self.initialized {
            return Err(vortex_types::VortexError::HardwareError);
        }

        // First, write register address
        self.write(addr, &[reg])?;
        
        // Set slave address for read
        mmio::mmio_write(0x0804_0008, ((addr.0 as u32) << 1) | 0x01);
        
        // Set read length
        mmio::mmio_write(0x0804_000C, buffer.len() as u32);
        
        // Start read
        mmio::mmio_write(0x0804_0004, 0x80);  // Set I2CEN (read mode)
        
        // Read data from FIFO
        for entry in buffer.iter_mut() {
            loop {
                let status = mmio::mmio_read(0x0804_0004);
                if (status & 0x20) != 0 {  // RXD bit (RX FIFO has data)
                    *entry = mmio::mmio_read(0x0804_0040) as u8;
                    break;
                }
            }
        }
        
        Ok(())
    }

    fn write_read(&mut self, addr: I2cAddress, write_data: &[u8], read_buffer: &mut [u8]) -> VortexResult<()> {
        self.write(addr, write_data)?;
        self.read(addr, write_data[0], read_buffer)?;
        Ok(())
    }
}

pub fn init() -> VortexResult<()> {
    Ok(())
}
