//! Vortex Microkernel - Real-time OS for Drones, Cars, and Robots
//! 
//! Core Features:
//! - O(1) Real-time Scheduler with 32 priority levels
//! - Pre-emptive scheduling with priority inversion prevention
//! - Lightweight IPC with message passing
//! - Virtual memory and protection domains
//! - Interrupt handling and routing

#![no_std]
#![warn(missing_docs)]

extern crate alloc;

pub mod scheduler;
pub mod memory;
pub mod ipc;
pub mod interrupt;
pub mod sync;
pub mod task;

use vortex_types::VortexResult;

/// Initialize the microkernel
pub fn init() -> VortexResult<()> {
    memory::init()?;
    scheduler::init()?;
    interrupt::init()?;
    Ok(())
}

/// Start the kernel scheduler (doesn't return)
pub fn start() -> ! {
    scheduler::start()
}