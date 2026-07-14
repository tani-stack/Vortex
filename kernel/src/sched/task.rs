#[derive(Debug, Clone)] pub struct Task { pub id: u64, pub priority: u8, pub state: TaskState }
#[derive(Debug, Clone, Copy, PartialEq)] pub enum TaskState { Ready, Running, Blocked, Faulted }
impl Task { pub fn new(id: u64, prio: u8) -> Self { Self{id, priority:prio, state:TaskState::Ready} } }
