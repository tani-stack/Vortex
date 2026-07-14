pub struct ImuData { pub accel: [f32;3], pub gyro: [f32;3], pub temp: f32 }
pub trait Imu: Send+Sync { fn read(&mut self) -> ImuData; fn is_healthy(&self)->bool { true } }
pub struct Icm42688; impl Imu for Icm42688 { fn read(&mut self)->ImuData { ImuData{accel:[0.0;3], gyro:[0.0;3], temp: 42.0} } }
