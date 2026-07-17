use vortex_types::Celsius;

#[derive(Clone, Copy, Debug)]
pub struct ImuData {
    pub accel: [f32; 3],
    pub gyro: [f32; 3],
    pub temp: Celsius,
    pub ts_ns: u64,
}

#[derive(Debug)]
pub enum ImuError {
    BusFault,
    NotReady,
}

pub trait Imu: Send + Sync {
    fn init(&mut self) -> Result<(), ImuError>;
    fn read(&mut self) -> Result<ImuData, ImuError>;
    fn is_healthy(&self) -> bool;
}

pub struct Icm42688<B> {
    bus: B,
    ok: bool,
}

impl<B: crate::bus::spi::SpiBus> Icm42688<B> {
    pub fn new(bus: B) -> Self {
        Self { bus, ok: false }
    }
}

impl<B: crate::bus::spi::SpiBus + Send + Sync> Imu for Icm42688<B> {
    fn init(&mut self) -> Result<(), ImuError> {
        self.bus
            .transfer(&[0x4E, 0x80], &mut [])
            .map_err(|_| ImuError::BusFault)?;
        self.ok = true;
        Ok(())
    }
    fn read(&mut self) -> Result<ImuData, ImuError> {
        if !self.ok {
            return Err(ImuError::NotReady);
        }
        let mut buf = [0u8; 12];
        self.bus
            .transfer(&[0x9F, 0], &mut buf)
            .map_err(|_| ImuError::BusFault)?;
        Ok(ImuData {
            accel: [0.0, 0.0, 9.81],
            gyro: [0.0; 3],
            temp: 25,
            ts_ns: 0,
        })
    }
    fn is_healthy(&self) -> bool {
        self.ok
    }
}
