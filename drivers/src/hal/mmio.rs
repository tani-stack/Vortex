//! Memory-Mapped I/O utilities for ARM64

use core::ptr::{read_volatile, write_volatile};

/// Safe MMIO read
pub fn mmio_read(addr: usize) -> u32 {
    unsafe { read_volatile(addr as *const u32) }
}

/// Safe MMIO write
pub fn mmio_write(addr: usize, value: u32) {
    unsafe { write_volatile(addr as *mut u32, value) }
}

/// Safe MMIO read/modify/write
pub fn mmio_modify(addr: usize, clear_mask: u32, set_mask: u32) {
    let val = mmio_read(addr);
    let new_val = (val & !clear_mask) | set_mask;
    mmio_write(addr, new_val);
}

/// ARM64 GIC (Generic Interrupt Controller) base
pub const GIC_BASE: usize = 0x0800_0000;

/// ARM64 UART0 base address (QEMU/Raspberry Pi style)
pub const UART0_BASE: usize = 0x0900_0000;

/// ARM64 GPIO base address
pub const GPIO_BASE: usize = 0x0A00_0000;

/// ARM64 Timer base
pub const TIMER_BASE: usize = 0x0B00_0000;
