//! UART (Universal Asynchronous Receiver-Transmitter) abstraction

use vortex_types::VortexResult;
use super::mmio;

/// UART Port trait
pub trait UartPort: Send + Sync {
    fn configure(&mut self, baudrate: u32) -> VortexResult<()>;
    fn write_byte(&mut self, byte: u8) -> VortexResult<()>;
    fn read_byte(&mut self) -> VortexResult<Option<u8>>;
    fn write(&mut self, data: &[u8]) -> VortexResult<()>;
    fn read(&mut self, buffer: &mut [u8]) -> VortexResult<usize>;
}

/// ARM64 UART0 implementation (PL011 style for QEMU/Raspberry Pi)
pub struct Arm64Uart0 {
    baudrate: u32,
    initialized: bool,
}

impl Arm64Uart0 {
    pub fn new() -> Self {
        Self {
            baudrate: 115200,
            initialized: false,
        }
    }
}

impl UartPort for Arm64Uart0 {
    fn configure(&mut self, baudrate: u32) -> VortexResult<()> {
        self.baudrate = baudrate;
        
        // PL011 UART configuration
        // UART0_IBRD (Integer Baud Rate Divider)
        let ibrd = 16 * 115200 / baudrate;  // Assuming 115200 clock
        mmio::mmio_write(mmio::UART0_BASE + 0x24, ibrd);
        
        // UART0_FBRD (Fractional Baud Rate Divider)
        let fbrd = ((64 * 115200) / baudrate) & 0x3F;
        mmio::mmio_write(mmio::UART0_BASE + 0x28, fbrd);
        
        // UART0_LCRH (Line Control Register) - 8 bits, 1 stop, no parity
        mmio::mmio_write(mmio::UART0_BASE + 0x2C, 0x70);
        
        // UART0_CR (Control Register) - Enable UART, TX, RX
        mmio::mmio_write(mmio::UART0_BASE + 0x30, 0x301);
        
        self.initialized = true;
        Ok(())
    }

    fn write_byte(&mut self, byte: u8) -> VortexResult<()> {
        if !self.initialized {
            return Err(vortex_types::VortexError::HardwareError);
        }
        
        // Wait for TX FIFO to have space
        loop {
            let flags = mmio::mmio_read(mmio::UART0_BASE + 0x18);  // FR register
            if (flags & 0x20) == 0 {  // TXFF bit (TX FIFO Full)
                break;
            }
        }
        
        // Write byte to data register
        mmio::mmio_write(mmio::UART0_BASE + 0x00, byte as u32);
        Ok(())
    }

    fn read_byte(&mut self) -> VortexResult<Option<u8>> {
        if !self.initialized {
            return Err(vortex_types::VortexError::HardwareError);
        }
        
        let flags = mmio::mmio_read(mmio::UART0_BASE + 0x18);  // FR register
        if (flags & 0x10) != 0 {  // RXFE bit (RX FIFO Empty)
            return Ok(None);
        }
        
        let byte = mmio::mmio_read(mmio::UART0_BASE + 0x00) as u8;
        Ok(Some(byte))
    }

    fn write(&mut self, data: &[u8]) -> VortexResult<()> {
        for &byte in data {
            self.write_byte(byte)?;
        }
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> VortexResult<usize> {
        let mut count = 0;
        for entry in buffer.iter_mut() {
            if let Some(byte) = self.read_byte()? {
                *entry = byte;
                count += 1;
            } else {
                break;
            }
        }
        Ok(count)
    }
}

pub fn init() -> VortexResult<()> {
    // Initialize UART subsystem
    Ok(())
}
