//! Interrupt Handling and Routing
//! 
//! Manages hardware and software interrupts

use aero_types::AeroResult;

/// Interrupt number
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InterruptNumber(pub u8);

impl InterruptNumber {
    /// Timer interrupt
    pub const TIMER: InterruptNumber = InterruptNumber(0);
    /// UART0 interrupt
    pub const UART0: InterruptNumber = InterruptNumber(32);
    /// UART1 interrupt
    pub const UART1: InterruptNumber = InterruptNumber(33);
    /// SPI interrupt
    pub const SPI: InterruptNumber = InterruptNumber(34);
    /// I2C interrupt
    pub const I2C: InterruptNumber = InterruptNumber(35);
    /// GPIO interrupt
    pub const GPIO: InterruptNumber = InterruptNumber(36);
}

/// Interrupt handler function type
pub type InterruptHandler = fn(InterruptNumber) -> AeroResult<()>;

/// Interrupt descriptor
pub struct InterruptDescriptor {
    /// Interrupt number
    pub number: InterruptNumber,
    /// Handler function
    pub handler: Option<InterruptHandler>,
    /// Priority
    pub priority: u8,
    /// Enabled
    pub enabled: bool,
}

impl InterruptDescriptor {
    /// Create a new interrupt descriptor
    pub fn new(number: InterruptNumber) -> Self {
        Self {
            number,
            handler: None,
            priority: 128,
            enabled: false,
        }
    }
}

/// Interrupt controller
pub struct InterruptController {
    /// Interrupt descriptors (256 interrupts)
    descriptors: [Option<InterruptDescriptor>; 256],
    /// Global interrupt enable flag
    enabled: bool,
}

impl InterruptController {
    /// Create a new interrupt controller
    pub fn new() -> Self {
        Self {
            descriptors: [None; 256],
            enabled: false,
        }
    }

    /// Register an interrupt handler
    pub fn register_handler(
        &mut self,
        irq: InterruptNumber,
        handler: InterruptHandler,
        priority: u8,
    ) -> AeroResult<()> {
        let idx = irq.0 as usize;
        let mut desc = InterruptDescriptor::new(irq);
        desc.handler = Some(handler);
        desc.priority = priority;
        self.descriptors[idx] = Some(desc);
        Ok(())
    }

    /// Enable an interrupt
    pub fn enable(&mut self, irq: InterruptNumber) -> AeroResult<()> {
        let idx = irq.0 as usize;
        if let Some(desc) = &mut self.descriptors[idx] {
            desc.enabled = true;
            Ok(())
        } else {
            Err(aero_types::AeroError::InvalidInterrupt)
        }
    }

    /// Disable an interrupt
    pub fn disable(&mut self, irq: InterruptNumber) -> AeroResult<()> {
        let idx = irq.0 as usize;
        if let Some(desc) = &mut self.descriptors[idx] {
            desc.enabled = false;
            Ok(())
        } else {
            Err(aero_types::AeroError::InvalidInterrupt)
        }
    }

    /// Handle an interrupt
    pub fn handle_interrupt(&mut self, irq: InterruptNumber) -> AeroResult<()> {
        let idx = irq.0 as usize;
        if let Some(Some(desc)) = self.descriptors.get_mut(idx) {
            if desc.enabled {
                if let Some(handler) = desc.handler {
                    return handler(irq);
                }
            }
        }
        Ok(())
    }

    /// Enable global interrupts
    pub fn enable_interrupts(&mut self) {
        self.enabled = true;
    }

    /// Disable global interrupts
    pub fn disable_interrupts(&mut self) {
        self.enabled = false;
    }

    /// Check if interrupts enabled
    pub fn are_enabled(&self) -> bool {
        self.enabled
    }
}

impl Default for InterruptController {
    fn default() -> Self {
        Self::new()
    }
}

static mut INTERRUPT_CONTROLLER: Option<InterruptController> = None;

/// Initialize interrupt controller
pub fn init() -> AeroResult<()> {
    unsafe {
        INTERRUPT_CONTROLLER = Some(InterruptController::new());
    }
    Ok(())
}

/// Get interrupt controller
pub fn interrupt_controller() -> &'static mut InterruptController {
    unsafe {
        INTERRUPT_CONTROLLER.as_mut().expect("Interrupt controller not initialized")
    }
}