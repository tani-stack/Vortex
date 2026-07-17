#![no_std]
pub const ABI_VERSION: u32 = 2;
#[repr(C)]
pub struct Syscall {
    pub num: u64,
    pub a0: u64,
    pub a1: u64,
    pub a2: u64,
}
pub const SYS_IPC_SEND: u64 = 1;
pub const SYS_IPC_RECV: u64 = 2;
pub const SYS_YIELD: u64 = 3;
#[repr(C)]
pub struct CapRights(pub u32);
impl CapRights {
    pub const SEND: Self = Self(1);
    pub const RECV: Self = Self(2);
}
