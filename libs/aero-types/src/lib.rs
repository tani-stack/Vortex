#![no_std]
pub type Hertz = u32; pub type Milliamps = u32; pub type Millivolts = u32; pub type Celsius = i16;
#[derive(Debug, Clone, Copy)] pub struct Quaternion { pub w: f32, pub x: f32, pub y: f32, pub z: f32 }
#[derive(Debug, Clone, Copy)] pub struct Vector3 { pub x: f32, pub y: f32, pub z: f32 }
