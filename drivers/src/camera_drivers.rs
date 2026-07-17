//! Camera Drivers
//! OV5640, OV7670, MT9D111, etc.

use vortex_types::VortexResult;

#[derive(Debug, Clone, Copy)]
pub struct Frame {
    pub width: u16,
    pub height: u16,
    pub format: ImageFormat,  // RGB565, JPEG, YUV420, etc.
    pub timestamp_ns: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    RGB565,
    JPEG,
    YUV420,
    RAW,
}

/// OV5640 Camera Module (5MP, for object detection)
pub struct Ov5640 {
    i2c_addr: u8,
    sccb_clk: u8,
    initialized: bool,
    resolution: (u16, u16),
}

impl Ov5640 {
    pub fn new(i2c_addr: u8, sccb_clk: u8) -> Self {
        Self {
            i2c_addr,
            sccb_clk,
            initialized: false,
            resolution: (2592, 1944),  // 5MP
        }
    }

    pub fn init(&mut self) -> VortexResult<()> {
        // Load OV5640 initialization sequence
        self.write_reg_seq()?;
        self.initialized = true;
        Ok(())
    }

    pub fn set_resolution(&mut self, width: u16, height: u16) -> VortexResult<()> {
        self.resolution = (width, height);
        Ok(())
    }

    pub fn capture(&mut self) -> VortexResult<Frame> {
        if !self.initialized {
            return Err(vortex_types::VortexError::HardwareError);
        }
        Ok(Frame {
            width: self.resolution.0,
            height: self.resolution.1,
            format: ImageFormat::JPEG,
            timestamp_ns: 0,
        })
    }

    fn write_reg_seq(&self) -> VortexResult<()> { Ok(()) }
}

/// OV7670 Camera Module (VGA, compact)
pub struct Ov7670 {
    i2c_addr: u8,
    initialized: bool,
}

impl Ov7670 {
    pub fn new(i2c_addr: u8) -> Self {
        Self {
            i2c_addr,
            initialized: false,
        }
    }

    pub fn init(&mut self) -> VortexResult<()> {
        self.initialized = true;
        Ok(())
    }

    pub fn capture(&mut self) -> VortexResult<Frame> {
        Ok(Frame {
            width: 640,
            height: 480,
            format: ImageFormat::RGB565,
            timestamp_ns: 0,
        })
    }
}

/// MT9D111 Camera Module (2MP)
pub struct Mt9d111 {
    i2c_addr: u8,
    initialized: bool,
}

impl Mt9d111 {
    pub fn new(i2c_addr: u8) -> Self {
        Self {
            i2c_addr,
            initialized: false,
        }
    }

    pub fn init(&mut self) -> VortexResult<()> {
        self.initialized = true;
        Ok(())
    }

    pub fn capture(&mut self) -> VortexResult<Frame> {
        Ok(Frame {
            width: 1600,
            height: 1200,
            format: ImageFormat::JPEG,
            timestamp_ns: 0,
        })
    }
}
