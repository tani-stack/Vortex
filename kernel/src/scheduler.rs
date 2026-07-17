//! Real-Time Scheduler - O(1) with Priority Levels
//! 
//! Implements a priority-based preemptive scheduler with O(1) time complexity.
//! Supports 32 priority levels (0 = lowest, 31 = highest).

use alloc::vec::Vec;
use alloc::collections::VecDeque;
use aero_types::AeroResult;
use crate::task::{TaskId, TaskControlBlock, Priority, TaskState};

/// Priority queue (bitmap-based for O(1) scheduling)
pub struct PriorityQueue {
    /// Ready queues for each priority level
    queues: [VecDeque<TaskId>; 32],
    /// Bitmap of non-empty queues (bit i = 1 if queue i has tasks)
    bitmap: u32,
}

impl PriorityQueue {
    /// Create a new priority queue
    pub fn new() -> Self {
        Self {
            queues: Default::default(),
            bitmap: 0,
        }
    }

    /// Add a task to the appropriate queue
    pub fn enqueue(&mut self, task_id: TaskId, priority: Priority) {
        let p = priority.0 as usize;
        self.queues[p].push_back(task_id);
        self.bitmap |= 1 << p;
    }

    /// Remove a task from a queue
    pub fn dequeue(&mut self, task_id: TaskId, priority: Priority) -> bool {
        let p = priority.0 as usize;
        if let Some(pos) = self.queues[p].iter().position(|&id| id == task_id) {
            self.queues[p].remove(pos);
            if self.queues[p].is_empty() {
                self.bitmap &= !(1 << p);
            }
            return true;
        }
        false
    }

    /// Get the next runnable task (highest priority first) - O(1)
    pub fn next_runnable(&mut self) -> Option<TaskId> {
        if self.bitmap == 0 {
            return None;
        }
        
        let highest_priority = 31 - self.bitmap.leading_zeros() as usize;
        
        if let Some(task_id) = self.queues[highest_priority].pop_front() {
            if self.queues[highest_priority].is_empty() {
                self.bitmap &= !(1 << highest_priority);
            }
            return Some(task_id);
        }
        None
    }

    /// Get all ready tasks at a priority level
    pub fn get_at_priority(&self, priority: Priority) -> Vec<TaskId> {
        let p = priority.0 as usize;
        self.queues[p].iter().copied().collect()
    }

    /// Count of ready tasks
    pub fn count(&self) -> usize {
        self.queues.iter().map(|q| q.len()).sum()
    }
}

impl Default for PriorityQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Global scheduler state
pub struct Scheduler {
    /// Priority queue for ready tasks
    ready_queue: PriorityQueue,
    /// Currently running task
    current_task: Option<TaskId>,
    /// Task registry
    tasks: Vec<Option<TaskControlBlock>>,
    /// Next task ID
    next_id: u32,
    /// Schedule count
    schedule_count: u64,
}

impl Scheduler {
    /// Create a new scheduler
    pub fn new() -> Self {
        Self {
            ready_queue: PriorityQueue::new(),
            current_task: None,
            tasks: Vec::new(),
            next_id: 1,
            schedule_count: 0,
        }
    }

    /// Initialize scheduler
    pub fn init() -> AeroResult<()> {
        Ok(())
    }

    /// Create and register a new task
    pub fn create_task(
        &mut self,
        name: alloc::string::String,
        priority: Priority,
        entry: extern "C" fn(),
        stack_size: usize,
    ) -> AeroResult<TaskId> {
        let id = TaskId(self.next_id);
        self.next_id += 1;

        let tcb = TaskControlBlock::new(id, name, priority, entry, stack_size);
        self.tasks.push(Some(tcb));
        self.ready_queue.enqueue(id, priority);

        Ok(id)
    }

    /// Get current running task
    pub fn current_task(&self) -> Option<TaskId> {
        self.current_task
    }

    /// Yield current task to scheduler
    pub fn yield_task(&mut self) {
        if let Some(current_id) = self.current_task {
            if let Some(Some(task)) = self.tasks.get_mut(current_id.0 as usize) {
                if task.state == TaskState::Running {
                    task.state = TaskState::Ready;
                    self.ready_queue.enqueue(current_id, task.priority);
                }
            }
        }
        self.schedule();
    }

    /// Schedule the next task (called on timer interrupt or context switch)
    pub fn schedule(&mut self) {
        if let Some(current_id) = self.current_task {
            if let Some(Some(task)) = self.tasks.get_mut(current_id.0 as usize) {
                task.state = TaskState::Ready;
                self.ready_queue.enqueue(current_id, task.priority);
            }
        }

        if let Some(next_id) = self.ready_queue.next_runnable() {
            if let Some(Some(task)) = self.tasks.get_mut(next_id.0 as usize) {
                task.state = TaskState::Running;
                task.reset_time_slice();
                self.current_task = Some(next_id);
                self.schedule_count += 1;
            }
        }
    }

    /// Block current task (waiting for event)
    pub fn block_task(&mut self, task_id: TaskId) -> AeroResult<()> {
        if let Some(Some(task)) = self.tasks.get_mut(task_id.0 as usize) {
            self.ready_queue.dequeue(task_id, task.priority);
            task.state = TaskState::Blocked;
        }
        self.schedule();
        Ok(())
    }

    /// Unblock a task
    pub fn unblock_task(&mut self, task_id: TaskId) -> AeroResult<()> {
        if let Some(Some(task)) = self.tasks.get_mut(task_id.0 as usize) {
            if task.state == TaskState::Blocked {
                task.state = TaskState::Ready;
                self.ready_queue.enqueue(task_id, task.priority);
            }
        }
        Ok(())
    }

    /// Get task by ID
    pub fn get_task(&self, task_id: TaskId) -> Option<&TaskControlBlock> {
        self.tasks.get(task_id.0 as usize).and_then(|t| t.as_ref())
    }

    /// Get mutable task by ID
    pub fn get_task_mut(&mut self, task_id: TaskId) -> Option<&mut TaskControlBlock> {
        self.tasks.get_mut(task_id.0 as usize).and_then(|t| t.as_mut())
    }

    /// Get scheduler statistics
    pub fn stats(&self) -> SchedulerStats {
        SchedulerStats {
            total_tasks: self.tasks.len(),
            ready_tasks: self.ready_queue.count(),
            schedule_count: self.schedule_count,
        }
    }
}

/// Scheduler statistics
#[derive(Debug, Clone, Copy)]
pub struct SchedulerStats {
    /// Total tasks created
    pub total_tasks: usize,
    /// Ready to run tasks
    pub ready_tasks: usize,
    /// Times scheduler ran
    pub schedule_count: u64,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

static mut SCHEDULER: Option<Scheduler> = None;

/// Initialize the global scheduler
pub fn init() -> AeroResult<()> {
    unsafe {
        SCHEDULER = Some(Scheduler::new());
    }
    Ok(())
}

/// Get current scheduler
pub fn scheduler() -> &'static mut Scheduler {
    unsafe {
        SCHEDULER.as_mut().expect("Scheduler not initialized")
    }
}

/// Start the kernel scheduler (doesn't return)
pub fn start() -> ! {
    let sched = scheduler();
    sched.schedule();
    
    loop {
        core::hint::spin_loop();
    }
}