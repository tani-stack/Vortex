use super::task::Task;
use alloc::collections::VecDeque;
use spin::Mutex;
static RUNQ: Mutex<VecDeque<Task>> = Mutex::new(VecDeque::new());
pub fn init() {}
pub fn schedule() { let mut q=RUNQ.lock(); if let Some(t)=q.pop_front(){ q.push_back(t); } }
pub fn spawn(t: Task) { RUNQ.lock().push_back(t); }
