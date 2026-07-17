//! Hardware Abstraction Layer for VORTEX OS
//! Provides platform-independent hardware interfaces

pub mod gpio;
pub mod pwm;
pub mod adc;
pub mod uart;
pub mod spi;
pub mod i2c;
pub mod timer;
pub mod interrupt;

/// Initialize HAL for the target platform
pub fn init() {
    gpio::init();
    timer::init();
    uart::init();
    spi::init();
    i2c::init();
    adc::init();
    interrupt::init();
}
