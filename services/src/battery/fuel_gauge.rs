pub trait FuelGauge { fn soc(&mut self)->u8; fn voltage(&mut self)->u32; }
