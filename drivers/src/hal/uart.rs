//! UART (Universal Asynchronous Receiver-Transmitter) HAL

use vortex_types::VortexResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UARTBaud {
    B9600,
    B19200,
    B38400,
    B57600,
    B115200,
}

impl UARTBaud {
    pub fn as_u32(&self) -> u32 {
        match self {
            UARTBaud::B9600 => 9600,
            UARTBaud::B19200 => 19200,
            UARTBaud::B38400 => 38400,
            UARTBaud::B57600 => 57600,
            UARTBaud::B115200 => 115200,
        }
    }
}

pub struct UART {
    port: u8,
    baud: UARTBaud,
}

impl UART {
    pub fn new(port: u8, baud: UARTBaud) -> VortexResult<Self> {
        Self::init_uart(port, baud)?;
        Ok(Self { port, baud })
    }

    pub fn set_baud(&mut self, baud: UARTBaud) -> VortexResult<()> {
        Self::configure_baud(self.port, baud)?;
        self.baud = baud;
        Ok(())
    }

    pub fn write_byte(&self, byte: u8) -> VortexResult<()> {
        Self::tx_byte(self.port, byte)?;
        Ok(())
    }

    pub fn write_bytes(&self, data: &[u8]) -> VortexResult<()> {
        for &byte in data {
            Self::tx_byte(self.port, byte)?;
        }
        Ok(())
    }

    pub fn read_byte(&self) -> VortexResult<Option<u8>> {
        Self::rx_byte(self.port)
    }

    pub fn read_available(&self, buf: &mut [u8]) -> VortexResult<usize> {
        let mut count = 0;
        while count < buf.len() {
            match Self::rx_byte(self.port)? {
                Some(byte) => {
                    buf[count] = byte;
                    count += 1;
                }
                None => break,
            }
        }
        Ok(count)
    }

    pub fn write_string(&self, s: &str) -> VortexResult<()> {
        self.write_bytes(s.as_bytes())
    }

    #[inline(always)]
    fn init_uart(port: u8, _baud: UARTBaud) -> VortexResult<()> {
        let _ = port;
        Ok(())
    }

    #[inline(always)]
    fn configure_baud(port: u8, _baud: UARTBaud) -> VortexResult<()> {
        let _ = port;
        Ok(())
    }

    #[inline(always)]
    fn tx_byte(port: u8, _byte: u8) -> VortexResult<()> {
        let _ = port;
        Ok(())
    }

    #[inline(always)]
    fn rx_byte(port: u8) -> VortexResult<Option<u8>> {
        let _ = port;
        Ok(None)
    }
}

pub fn init() {}
