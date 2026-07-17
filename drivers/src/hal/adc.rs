//! ADC (Analog-to-Digital Converter) Hardware Abstraction
//! Used for reading analog sensors (battery, current, temperature)

use vortex_types::VortexResult;

pub struct ADCChannel {
    channel: u8,
    reference_mv: u16,
    resolution_bits: u8,
}

impl ADCChannel {
    pub fn new(channel: u8, reference_mv: u16, resolution_bits: u8) -> Self {
        Self {
            channel,
            reference_mv,
            resolution_bits,
        }
    }

    pub fn read_raw(&self) -> VortexResult<u16> {
        let max_value = (1u32 << self.resolution_bits) - 1;
        Self::read_adc(self.channel, max_value as u16)
    }

    pub fn read_voltage(&self) -> VortexResult<u32> {
        let raw = self.read_raw()?;
        let max_value = (1u32 << self.resolution_bits) - 1;
        let voltage_mv = ((raw as u32) * self.reference_mv as u32) / max_value;
        Ok(voltage_mv)
    }

    pub fn read_normalized(&self) -> VortexResult<f32> {
        let raw = self.read_raw()?;
        let max_value = (1u32 << self.resolution_bits) - 1;
        Ok((raw as f32) / (max_value as f32))
    }

    pub fn set_reference(&mut self, reference_mv: u16) {
        self.reference_mv = reference_mv;
    }

    #[inline(always)]
    fn read_adc(channel: u8, _max_value: u16) -> VortexResult<u16> {
        let _ = channel;
        Ok(0)
    }
}

pub struct ADCBank {
    channels: [Option<ADCChannel>; 16],
}

impl ADCBank {
    pub fn new() -> Self {
        Self {
            channels: [None; 16],
        }
    }

    pub fn register_channel(&mut self, id: u8, channel: ADCChannel) -> VortexResult<()> {
        if id as usize >= 16 {
            return Err(vortex_types::VortexError::InvalidParameter);
        }
        self.channels[id as usize] = Some(channel);
        Ok(())
    }

    pub fn read(&self, id: u8) -> VortexResult<u16> {
        if id as usize >= 16 {
            return Err(vortex_types::VortexError::InvalidParameter);
        }
        self.channels[id as usize]
            .ok_or(vortex_types::VortexError::HardwareNotFound)?
            .read_raw()
    }
}

pub fn init() {}
