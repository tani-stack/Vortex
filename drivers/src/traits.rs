pub trait Driver: Send + Sync { fn init(&mut self) -> Result<(), ()>; fn health(&self) -> bool; fn name(&self) -> &'static str; }
