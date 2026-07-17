//! Task and Thread Management
//! 
//! Defines task structures, states, and lifecycle management

use alloc::string::String;
use core::fmt;

/// Task/Thread ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskId(pub u32);

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Task#{}", self.0)
    }
}

/// Task state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskState {
    /// Ready to run
    Ready,
    /// Currently running
    Running,
    /// Waiting for an event
    Waiting,
    /// Suspended (not in scheduler queue)
    Suspended,
    /// Task has terminated
    Terminated,
    /// Blocked on synchronization primitive
    Blocked,
}

impl fmt::Display for TaskState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskState::Ready => write!(f, "Ready"),
            TaskState::Running => write!(f, "Running"),
            TaskState::Waiting => write!(f, "Waiting"),
            TaskState::Suspended => write!(f, "Suspended"),
            TaskState::Terminated => write!(f, "Terminated"),
            TaskState::Blocked => write!(f, "Blocked"),
        }
    }
}

/// Task priority (0-31, where 31 is highest)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Priority(pub u8);

impl Priority {
    /// Highest priority (for critical real-time tasks)
    pub const HIGHEST: Priority = Priority(31);
    /// High priority (drones, critical timing)
    pub const HIGH: Priority = Priority(24);
    /// Normal priority
    pub const NORMAL: Priority = Priority(16);
    /// Low priority (background tasks)
    pub const LOW: Priority = Priority(8);
    /// Lowest priority (idle)
    pub const LOWEST: Priority = Priority(0);

    /// Validate priority (0-31)
    pub fn new(p: u8) -> Option<Priority> {
        if p < 32 {
            Some(Priority(p))
        } else {
            None
        }
    }
}

/// Task control block
#[derive(Debug)]
pub struct TaskControlBlock {
    /// Unique task ID
    pub id: TaskId,
    /// Task name
    pub name: String,
    /// Current state
    pub state: TaskState,
    /// Priority level
    pub priority: Priority,
    /// Stack pointer
    pub sp: u32,
    /// Program counter
    pub pc: u32,
    /// Saved context (registers)
    pub context: TaskContext,
    /// Time allocated (in ms, 0 = unlimited)
    pub time_slice: u32,
    /// Time used in current slice
    pub time_used: u32,
    /// Entry point function pointer
    pub entry: extern "C" fn(),
    /// Stack size
    pub stack_size: usize,
    /// Creation timestamp (ns)
    pub created_at: u64,
    /// Last scheduled timestamp (ns)
    pub last_scheduled: u64,
}

/// Saved CPU context for a task
#[derive(Debug, Clone, Copy)]
pub struct TaskContext {
    /// General purpose registers
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r4: u32,
    pub r5: u32,
    pub r6: u32,
    pub r7: u32,
    pub r8: u32,
    pub r9: u32,
    pub r10: u32,
    pub r11: u32,
    pub r12: u32,
    /// Stack pointer
    pub sp: u32,
    /// Link register
    pub lr: u32,
    /// Program counter
    pub pc: u32,
    /// Program status register
    pub psr: u32,
}

impl Default for TaskContext {
    fn default() -> Self {
        Self {
            r0: 0, r1: 0, r2: 0, r3: 0, r4: 0, r5: 0, r6: 0, r7: 0,
            r8: 0, r9: 0, r10: 0, r11: 0, r12: 0, sp: 0, lr: 0, pc: 0, psr: 0,
        }
    }
}

impl TaskControlBlock {
    /// Create a new task
    pub fn new(
        id: TaskId,
        name: String,
        priority: Priority,
        entry: extern "C" fn(),
        stack_size: usize,
    ) -> Self {
        Self {
            id,
            name,
            state: TaskState::Ready,
            priority,
            sp: 0,
            pc: entry as u32,
            context: TaskContext::default(),
            time_slice: 10,
            time_used: 0,
            entry,
            stack_size,
            created_at: 0,
            last_scheduled: 0,
        }
    }

    /// Check if task is runnable
    pub fn is_runnable(&self) -> bool {
        matches!(self.state, TaskState::Ready | TaskState::Running)
    }

    /// Reset time slice usage
    pub fn reset_time_slice(&mut self) {
        self.time_used = 0;
    }

    /// Check if time slice expired
    pub fn time_slice_expired(&self) -> bool {
        self.time_slice > 0 && self.time_used >= self.time_slice
    }
}