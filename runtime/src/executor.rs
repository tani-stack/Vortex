use core::future::Future; use core::pin::Pin; use core::task::{Context, Poll};
pub struct NoAllocExecutor;
impl NoAllocExecutor { pub fn spawn<F: Future<Output=()> + 'static>(&self, _f: F) {} pub fn run(&self) -> ! { loop { core::hint::spin_loop(); } } }
