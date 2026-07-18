//! Hardware Abstraction Layer (HAL)
//! Trait-based interface for hardware operations
//! Supports GPIO, UART, I2C, SPI, PWM, ADC, Timers

pub mod gpio;
pub mod uart;
pub mod i2c;
pub mod spi;
pub mod pwm;
pub mod adc;
pub mod timer;
pub mod mmio;

use vortex_types::VortexResult;

/// Initialize all HAL modules
pub fn init() -> VortexResult<()> {
    gpio::init()?;
    uart::init()?;
    i2c::init()?;
    spi::init()?;
    pwm::init()?;
    adc::init()?;
    timer::init()?;
    Ok(())
}
