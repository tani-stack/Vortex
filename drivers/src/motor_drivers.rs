//! Motor and Motor Driver ICs
//! L298N, DRV8825, TMC2209, VESCs, etc.

use vortex_types::VortexResult;

/// Motor Command
#[derive(Debug, Clone, Copy)]
pub struct MotorCommand {
    pub speed: i16,  // -1000 to 1000
    pub brake: bool,
}

/// L298N DC Motor Driver (Cars, Robots)
pub struct L298n {
    pwm_pin_a: u8,
    pwm_pin_b: u8,
    enable_pin: u8,
    initialized: bool,
}

impl L298n {
    pub fn new(pwm_a: u8, pwm_b: u8, enable: u8) -> Self {
        Self {
            pwm_pin_a: pwm_a,
            pwm_pin_b: pwm_b,
            enable_pin: enable,
            initialized: false,
        }
    }

    pub fn init(&mut self) -> VortexResult<()> {
        self.initialized = true;
        Ok(())
    }

    pub fn set_speed(&mut self, cmd: MotorCommand) -> VortexResult<()> {
        if !self.initialized {
            return Err(vortex_types::VortexError::HardwareError);
        }

        let speed = cmd.speed.clamp(-1000, 1000);
        let duty = ((speed.abs() as u32 * 255) / 1000) as u8;

        if speed > 0 {
            self.set_pin_high(self.pwm_pin_a)?;
            self.set_pin_low(self.pwm_pin_b)?;
        } else if speed < 0 {
            self.set_pin_low(self.pwm_pin_a)?;
            self.set_pin_high(self.pwm_pin_b)?;
        } else {
            self.set_pin_low(self.pwm_pin_a)?;
            self.set_pin_low(self.pwm_pin_b)?;
        }

        self.set_pwm(self.enable_pin, duty)?;
        Ok(())
    }

    fn set_pin_high(&self, pin: u8) -> VortexResult<()> { Ok(()) }
    fn set_pin_low(&self, pin: u8) -> VortexResult<()> { Ok(()) }
    fn set_pwm(&self, pin: u8, duty: u8) -> VortexResult<()> { Ok(()) }
}

/// DRV8825 Stepper Motor Driver
pub struct Drv8825 {
    step_pin: u8,
    dir_pin: u8,
    enable_pin: u8,
    position: i32,
}

impl Drv8825 {
    pub fn new(step: u8, dir: u8, enable: u8) -> Self {
        Self {
            step_pin: step,
            dir_pin: dir,
            enable_pin: enable,
            position: 0,
        }
    }

    pub fn init(&mut self) -> VortexResult<()> { Ok(()) }

    pub fn step(&mut self, direction: bool, steps: u16) -> VortexResult<()> {
        self.set_direction(direction)?;
        
        for _ in 0..steps {
            self.pulse_step()?;
            if direction {
                self.position += 1;
            } else {
                self.position -= 1;
            }
        }
        Ok(())
    }

    fn set_direction(&self, dir: bool) -> VortexResult<()> { Ok(()) }
    fn pulse_step(&self) -> VortexResult<()> { Ok(()) }
}

/// TMC2209 Silent Stepper Driver
pub struct Tmc2209 {
    uart_port: u8,
    slave_addr: u8,
    position: i32,
}

impl Tmc2209 {
    pub fn new(uart_port: u8, addr: u8) -> Self {
        Self {
            uart_port,
            slave_addr: addr,
            position: 0,
        }
    }

    pub fn init(&mut self) -> VortexResult<()> { Ok(()) }

    pub fn move_to(&mut self, target: i32) -> VortexResult<()> {
        self.position = target;
        Ok(())
    }
}

/// ESC (Electronic Speed Controller) for brushless motors
pub struct Esc {
    pwm_pin: u8,
    min_throttle: u16,
    max_throttle: u16,
    armed: bool,
}

impl Esc {
    pub fn new(pwm_pin: u8) -> Self {
        Self {
            pwm_pin,
            min_throttle: 1000,
            max_throttle: 2000,
            armed: false,
        }
    }

    pub fn init(&mut self) -> VortexResult<()> { Ok(()) }

    pub fn arm(&mut self) -> VortexResult<()> {
        self.armed = true;
        Ok(())
    }

    pub fn disarm(&mut self) -> VortexResult<()> {
        self.armed = false;
        Ok(())
    }

    pub fn set_throttle(&mut self, throttle: u16) -> VortexResult<()> {
        if !self.armed {
            return Err(vortex_types::VortexError::HardwareError);
        }
        let clamped = throttle.clamp(self.min_throttle, self.max_throttle);
        self.set_pwm(clamped)?;
        Ok(())
    }

    fn set_pwm(&self, pwm: u16) -> VortexResult<()> { Ok(()) }
}

/// Servo Motor
pub struct Servo {
    pwm_pin: u8,
    angle: f32,
}

impl Servo {
    pub fn new(pwm_pin: u8) -> Self {
        Self {
            pwm_pin,
            angle: 90.0,
        }
    }

    pub fn init(&mut self) -> VortexResult<()> { Ok(()) }

    pub fn set_angle(&mut self, angle: f32) -> VortexResult<()> {
        let clamped = angle.clamp(0.0, 180.0);
        let pwm = ((clamped / 180.0) * 1000.0) + 1000.0;  // 1000-2000 us
        self.angle = clamped;
        self.set_pwm(pwm as u16)?;
        Ok(())
    }

    fn set_pwm(&self, pwm: u16) -> VortexResult<()> { Ok(()) }
}
