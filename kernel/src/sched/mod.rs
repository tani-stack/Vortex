pub mod scheduler; pub mod task; pub mod idle; pub fn init() { scheduler::init(); } pub fn tick() { scheduler::schedule(); }
