pub struct Watchdog; impl Watchdog { pub fn kick(&mut self) {} pub fn is_expired(&self)->bool { false } }
