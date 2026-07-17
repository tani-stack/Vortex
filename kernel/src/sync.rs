//! Synchronization Primitives
//! 
//! Provides mutex, semaphore, and barrier implementations

use alloc::collections::VecDeque;
use aero_types::AeroResult;
use crate::task::TaskId;

/// Spinlock - Simple lock with busy waiting
pub struct Spinlock {
    /// Locked flag
    locked: bool,
}

impl Spinlock {
    /// Create a new spinlock
    pub const fn new() -> Self {
        Self { locked: false }
    }

    /// Try to acquire lock (non-blocking)
    pub fn try_lock(&mut self) -> bool {
        if !self.locked {
            self.locked = true;
            true
        } else {
            false
        }
    }

    /// Acquire lock (busy waiting)
    pub fn lock(&mut self) {
        while !self.try_lock() {
            core::hint::spin_loop();
        }
    }

    /// Release lock
    pub fn unlock(&mut self) {
        self.locked = false;
    }

    /// Check if locked
    pub fn is_locked(&self) -> bool {
        self.locked
    }
}

impl Default for Spinlock {
    fn default() -> Self {
        Self::new()
    }
}

/// Mutex - Blocking mutual exclusion lock
pub struct Mutex {
    /// Lock holder
    holder: Option<TaskId>,
    /// Waiting queue
    waiters: VecDeque<TaskId>,
    /// Lock count (for recursive mutex)
    count: u32,
}

impl Mutex {
    /// Create a new mutex
    pub fn new() -> Self {
        Self {
            holder: None,
            waiters: VecDeque::new(),
            count: 0,
        }
    }

    /// Lock the mutex
    pub fn lock(&mut self, task_id: TaskId) -> AeroResult<()> {
        if self.holder == Some(task_id) {
            self.count += 1;
            return Ok(());
        }

        if self.holder.is_none() {
            self.holder = Some(task_id);
            self.count = 1;
            Ok(())
        } else {
            self.waiters.push_back(task_id);
            Err(aero_types::AeroError::MutexLocked)
        }
    }

    /// Try to lock (non-blocking)
    pub fn try_lock(&mut self, task_id: TaskId) -> AeroResult<bool> {
        if self.holder == Some(task_id) {
            self.count += 1;
            Ok(true)
        } else if self.holder.is_none() {
            self.holder = Some(task_id);
            self.count = 1;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Unlock the mutex
    pub fn unlock(&mut self) -> AeroResult<()> {
        if self.count > 1 {
            self.count -= 1;
            return Ok(());
        }

        if self.holder.is_some() {
            if let Some(next) = self.waiters.pop_front() {
                self.holder = Some(next);
                self.count = 1;
            } else {
                self.holder = None;
                self.count = 0;
            }
            Ok(())
        } else {
            Err(aero_types::AeroError::OperationFailed)
        }
    }

    /// Get lock holder
    pub fn holder(&self) -> Option<TaskId> {
        self.holder
    }

    /// Count of waiting tasks
    pub fn waiters_count(&self) -> usize {
        self.waiters.len()
    }
}

impl Default for Mutex {
    fn default() -> Self {
        Self::new()
    }
}

/// Semaphore - Counter-based synchronization
pub struct Semaphore {
    /// Semaphore count
    count: u32,
    /// Waiting tasks
    waiters: VecDeque<TaskId>,
}

impl Semaphore {
    /// Create a new semaphore
    pub fn new(initial_count: u32) -> Self {
        Self {
            count: initial_count,
            waiters: VecDeque::new(),
        }
    }

    /// Signal (increment)
    pub fn signal(&mut self) -> AeroResult<()> {
        if self.count < u32::MAX {
            self.count += 1;
            Ok(())
        } else {
            Err(aero_types::AeroError::OperationFailed)
        }
    }

    /// Wait (decrement if available)
    pub fn wait(&mut self, task_id: TaskId) -> AeroResult<()> {
        if self.count > 0 {
            self.count -= 1;
            Ok(())
        } else {
            self.waiters.push_back(task_id);
            Err(aero_types::AeroError::WouldBlock)
        }
    }

    /// Try to wait (non-blocking)
    pub fn try_wait(&mut self) -> bool {
        if self.count > 0 {
            self.count -= 1;
            true
        } else {
            false
        }
    }

    /// Get count
    pub fn count(&self) -> u32 {
        self.count
    }
}

impl Default for Semaphore {
    fn default() -> Self {
        Self::new(1)
    }
}

/// Barrier - Synchronization point for multiple tasks
pub struct Barrier {
    /// Total participants
    total: u32,
    /// Current count
    current: u32,
    /// Waiting tasks
    waiters: VecDeque<TaskId>,
}

impl Barrier {
    /// Create a new barrier
    pub fn new(participants: u32) -> Self {
        Self {
            total: participants,
            current: 0,
            waiters: VecDeque::new(),
        }
    }

    /// Wait at barrier
    pub fn wait(&mut self, task_id: TaskId) -> AeroResult<bool> {
        self.current += 1;
        
        if self.current >= self.total {
            self.waiters.clear();
            self.current = 0;
            Ok(true)
        } else {
            self.waiters.push_back(task_id);
            Err(aero_types::AeroError::WouldBlock)
        }
    }

    /// Get current count
    pub fn current_count(&self) -> u32 {
        self.current
    }

    /// Get total participants
    pub fn total(&self) -> u32 {
        self.total
    }
}

/// Read-Write Lock
pub struct RwLock {
    /// Read count
    readers: u32,
    /// Write locked
    writer: Option<TaskId>,
    /// Waiting readers
    waiting_readers: VecDeque<TaskId>,
    /// Waiting writers
    waiting_writers: VecDeque<TaskId>,
}

impl RwLock {
    /// Create a new RW lock
    pub fn new() -> Self {
        Self {
            readers: 0,
            writer: None,
            waiting_readers: VecDeque::new(),
            waiting_writers: VecDeque::new(),
        }
    }

    /// Read lock
    pub fn read_lock(&mut self, task_id: TaskId) -> AeroResult<()> {
        if self.writer.is_none() && self.waiting_writers.is_empty() {
            self.readers += 1;
            Ok(())
        } else {
            self.waiting_readers.push_back(task_id);
            Err(aero_types::AeroError::WouldBlock)
        }
    }

    /// Write lock
    pub fn write_lock(&mut self, task_id: TaskId) -> AeroResult<()> {
        if self.writer.is_none() && self.readers == 0 {
            self.writer = Some(task_id);
            Ok(())
        } else {
            self.waiting_writers.push_back(task_id);
            Err(aero_types::AeroError::WouldBlock)
        }
    }

    /// Read unlock
    pub fn read_unlock(&mut self) -> AeroResult<()> {
        if self.readers > 0 {
            self.readers -= 1;
            Ok(())
        } else {
            Err(aero_types::AeroError::OperationFailed)
        }
    }

    /// Write unlock
    pub fn write_unlock(&mut self) -> AeroResult<()> {
        if self.writer.is_some() {
            self.writer = None;
            Ok(())
        } else {
            Err(aero_types::AeroError::OperationFailed)
        }
    }
}

impl Default for RwLock {
    fn default() -> Self {
        Self::new()
    }
}