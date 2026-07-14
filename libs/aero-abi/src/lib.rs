pub const ABI_VERSION: u32 = 1; #[repr(C)] pub struct Syscall { pub num: u64, pub arg0: u64, pub arg1: u64 }
