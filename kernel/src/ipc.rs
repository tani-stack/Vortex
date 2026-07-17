//! Inter-Process Communication (IPC)
//! 
//! Provides message passing and capability-based access control

use alloc::collections::VecDeque;
use alloc::vec::Vec;
use aero_types::AeroResult;
use crate::task::TaskId;

/// Message
#[derive(Debug, Clone)]
pub struct Message {
    /// Sender task ID
    pub sender: TaskId,
    /// Message payload (up to 256 bytes)
    pub payload: [u8; 256],
    /// Payload length
    pub len: usize,
    /// Message type tag
    pub msg_type: u32,
}

impl Message {
    /// Create a new message
    pub fn new(sender: TaskId, msg_type: u32, payload: &[u8]) -> Self {
        let mut msg = Message {
            sender,
            payload: [0; 256],
            len: payload.len().min(256),
            msg_type,
        };
        msg.payload[..msg.len].copy_from_slice(&payload[..msg.len]);
        msg
    }
}

/// Message queue for a task
pub struct MessageQueue {
    /// Queued messages
    messages: VecDeque<Message>,
    /// Max size
    max_size: usize,
}

impl MessageQueue {
    /// Create a new message queue
    pub fn new(max_size: usize) -> Self {
        Self {
            messages: VecDeque::new(),
            max_size,
        }
    }

    /// Send a message (blocking if queue full)
    pub fn send(&mut self, msg: Message) -> AeroResult<()> {
        if self.messages.len() >= self.max_size {
            return Err(aero_types::AeroError::MessageQueueFull);
        }
        self.messages.push_back(msg);
        Ok(())
    }

    /// Receive a message (returns None if empty)
    pub fn receive(&mut self) -> Option<Message> {
        self.messages.pop_front()
    }

    /// Count of pending messages
    pub fn count(&self) -> usize {
        self.messages.len()
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

impl Default for MessageQueue {
    fn default() -> Self {
        Self::new(32)
    }
}

/// Capability - permission token for resource access
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capability {
    /// Capability ID
    pub id: u32,
    /// Resource ID being accessed
    pub resource_id: u32,
    /// Permission bits
    pub permissions: u32,
}

impl Capability {
    /// Read permission
    pub const READ: u32 = 1 << 0;
    /// Write permission
    pub const WRITE: u32 = 1 << 1;
    /// Execute permission
    pub const EXECUTE: u32 = 1 << 2;
    /// Delete permission
    pub const DELETE: u32 = 1 << 3;

    /// Create a new capability
    pub fn new(id: u32, resource_id: u32, permissions: u32) -> Self {
        Self {
            id,
            resource_id,
            permissions,
        }
    }

    /// Check if has permission
    pub fn has(&self, perm: u32) -> bool {
        (self.permissions & perm) != 0
    }

    /// Grant permission
    pub fn grant(&mut self, perm: u32) {
        self.permissions |= perm;
    }

    /// Revoke permission
    pub fn revoke(&mut self, perm: u32) {
        self.permissions &= !perm;
    }
}

/// Capability list for a task
pub struct CapabilityList {
    /// Capabilities
    capabilities: Vec<Capability>,
}

impl CapabilityList {
    /// Create a new capability list
    pub fn new() -> Self {
        Self {
            capabilities: Vec::new(),
        }
    }

    /// Add a capability
    pub fn add(&mut self, cap: Capability) {
        self.capabilities.push(cap);
    }

    /// Remove a capability
    pub fn remove(&mut self, cap_id: u32) -> bool {
        if let Some(pos) = self.capabilities.iter().position(|c| c.id == cap_id) {
            self.capabilities.remove(pos);
            return true;
        }
        false
    }

    /// Check if has capability
    pub fn has(&self, resource_id: u32, perm: u32) -> bool {
        self.capabilities
            .iter()
            .any(|c| c.resource_id == resource_id && c.has(perm))
    }

    /// Get capability
    pub fn get(&self, cap_id: u32) -> Option<&Capability> {
        self.capabilities.iter().find(|c| c.id == cap_id)
    }
}

impl Default for CapabilityList {
    fn default() -> Self {
        Self::new()
    }
}

/// IPC Manager
pub struct IpcManager {
    /// Message queues per task
    queues: Vec<Option<MessageQueue>>,
    /// Capability lists per task
    capabilities: Vec<Option<CapabilityList>>,
    /// Next capability ID
    next_cap_id: u32,
}

impl IpcManager {
    /// Create a new IPC manager
    pub fn new() -> Self {
        Self {
            queues: Vec::new(),
            capabilities: Vec::new(),
            next_cap_id: 1,
        }
    }

    /// Create message queue for a task
    pub fn create_queue(&mut self, task_id: TaskId, max_size: usize) {
        let idx = task_id.0 as usize;
        while self.queues.len() <= idx {
            self.queues.push(None);
        }
        self.queues[idx] = Some(MessageQueue::new(max_size));
    }

    /// Send a message to a task
    pub fn send_message(&mut self, to: TaskId, msg: Message) -> AeroResult<()> {
        let idx = to.0 as usize;
        if let Some(Some(queue)) = self.queues.get_mut(idx) {
            queue.send(msg)?;
            Ok(())
        } else {
            Err(aero_types::AeroError::TaskNotFound)
        }
    }

    /// Receive a message from a task's queue
    pub fn receive_message(&mut self, from: TaskId) -> AeroResult<Option<Message>> {
        let idx = from.0 as usize;
        if let Some(Some(queue)) = self.queues.get_mut(idx) {
            Ok(queue.receive())
        } else {
            Err(aero_types::AeroError::TaskNotFound)
        }
    }

    /// Grant a capability to a task
    pub fn grant_capability(
        &mut self,
        to: TaskId,
        resource_id: u32,
        permissions: u32,
    ) -> AeroResult<u32> {
        let cap_id = self.next_cap_id;
        self.next_cap_id += 1;

        let idx = to.0 as usize;
        while self.capabilities.len() <= idx {
            self.capabilities.push(None);
        }

        if self.capabilities[idx].is_none() {
            self.capabilities[idx] = Some(CapabilityList::new());
        }

        if let Some(Some(caps)) = self.capabilities.get_mut(idx) {
            caps.add(Capability::new(cap_id, resource_id, permissions));
            Ok(cap_id)
        } else {
            Err(aero_types::AeroError::OperationFailed)
        }
    }

    /// Check if task has capability
    pub fn check_capability(
        &self,
        task_id: TaskId,
        resource_id: u32,
        perm: u32,
    ) -> bool {
        let idx = task_id.0 as usize;
        if let Some(Some(caps)) = self.capabilities.get(idx) {
            caps.has(resource_id, perm)
        } else {
            false
        }
    }
}

impl Default for IpcManager {
    fn default() -> Self {
        Self::new()
    }
}

static mut IPC_MANAGER: Option<IpcManager> = None;

/// Initialize IPC manager
pub fn init() -> AeroResult<()> {
    unsafe {
        IPC_MANAGER = Some(IpcManager::new());
    }
    Ok(())
}

/// Get IPC manager
pub fn ipc_manager() -> &'static mut IpcManager {
    unsafe {
        IPC_MANAGER.as_mut().expect("IPC manager not initialized")
    }
}// High-speed inter-process communication
