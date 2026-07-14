pub type Handle = *mut u8; pub type Status = usize; pub struct SystemTable;
pub fn init_console(_: *mut SystemTable) {}
#[macro_export] macro_rules! println { ($($t:tt)*) => {{}} }
pub use println;
