//! Universal Device Registry - Android-style hardware abstraction
//! Central hub for all hardware devices (sensors, motors, actuators)
//! 
//! Features:
//! - Dynamic device registration/unregistration
//! - Auto-discovery and detection
//! - Hardware-agnostic implementations
//! - Plug-and-play support
//! - Real-time device health monitoring

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::boxed::Box;
use aero_types::{AeroError, AeroResult};
use core::fmt;

/// Unique device identifier (globally unique)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceId(pub u32);

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Device#{}", self.0)
    }
}

/// Device type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DeviceType {
    Sensor,
    Motor,
    Actuator,
    Communication,
    PowerManagement,
    Navigation,
    Accessory,
    Custom,
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeviceType::Sensor => write!(f, "Sensor"),
            DeviceType::Motor => write!(f, "Motor"),
            DeviceType::Actuator => write!(f, "Actuator"),
            DeviceType::Communication => write!(f, "Communication"),
            DeviceType::PowerManagement => write!(f, "PowerManagement"),
            DeviceType::Navigation => write!(f, "Navigation"),
            DeviceType::Accessory => write!(f, "Accessory"),
            DeviceType::Custom => write!(f, "Custom"),
        }
    }
}

/// Device operational status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceStatus {
    Uninitialized,
    Initializing,
    Ready,
    Running,
    Suspended,
    Error,
    Disconnected,
}

impl fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeviceStatus::Uninitialized => write!(f, "Uninitialized"),
            DeviceStatus::Initializing => write!(f, "Initializing"),
            DeviceStatus::Ready => write!(f, "Ready"),
            DeviceStatus::Running => write!(f, "Running"),
            DeviceStatus::Suspended => write!(f, "Suspended"),
            DeviceStatus::Error => write!(f, "Error"),
            DeviceStatus::Disconnected => write!(f, "Disconnected"),
        }
    }
}

/// Device health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Warning,
    Critical,
    Offline,
}

impl fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HealthStatus::Healthy => write!(f, "Healthy"),
            HealthStatus::Degraded => write!(f, "Degraded"),
            HealthStatus::Warning => write!(f, "Warning"),
            HealthStatus::Critical => write!(f, "Critical"),
            HealthStatus::Offline => write!(f, "Offline"),
        }
    }
}

/// Supported bus/protocol types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BusType {
    SPI,
    I2C,
    UART,
    CAN,
    PWM,
    GPIO,
    Analog,
    USB,
    Ethernet,
    I2S,
    OneWire,
    Custom(u8),
}

impl fmt::Display for BusType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BusType::SPI => write!(f, "SPI"),
            BusType::I2C => write!(f, "I2C"),
            BusType::UART => write!(f, "UART"),
            BusType::CAN => write!(f, "CAN"),
            BusType::PWM => write!(f, "PWM"),
            BusType::GPIO => write!(f, "GPIO"),
            BusType::Analog => write!(f, "Analog"),
            BusType::USB => write!(f, "USB"),
            BusType::Ethernet => write!(f, "Ethernet"),
            BusType::I2S => write!(f, "I2S"),
            BusType::OneWire => write!(f, "OneWire"),
            BusType::Custom(n) => write!(f, "Custom({})", n),
        }
    }
}

/// Device capabilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceCapabilities {
    pub readable: bool,
    pub writable: bool,
    pub configurable: bool,
    pub interruptible: bool,
    pub real_time: bool,
    pub dma_capable: bool,
}

impl DeviceCapabilities {
    pub const SENSOR_DEFAULT: Self = DeviceCapabilities {
        readable: true,
        writable: false,
        configurable: true,
        interruptible: true,
        real_time: true,
        dma_capable: false,
    };

    pub const MOTOR_DEFAULT: Self = DeviceCapabilities {
        readable: true,
        writable: true,
        configurable: true,
        interruptible: false,
        real_time: true,
        dma_capable: false,
    };

    pub const COMMUNICATION_DEFAULT: Self = DeviceCapabilities {
        readable: true,
        writable: true,
        configurable: true,
        interruptible: true,
        real_time: false,
        dma_capable: true,
    };
}

/// Device metadata
#[derive(Debug, Clone)]
pub struct DeviceMetadata {
    pub id: DeviceId,
    pub name: String,
    pub device_type: DeviceType,
    pub manufacturer: String,
    pub model: String,
    pub version: (u8, u8, u8),
    pub capabilities: DeviceCapabilities,
    pub bus_type: BusType,
    pub bus_address: u32,
    pub interrupt_line: Option<u8>,
    pub dma_channel: Option<u8>,
    pub sample_rate_hz: Option<u32>,
    pub power_consumption_mw: u16,
}

impl DeviceMetadata {
    pub fn new(
        id: DeviceId,
        name: String,
        device_type: DeviceType,
        manufacturer: String,
        model: String,
        bus_type: BusType,
        bus_address: u32,
    ) -> Self {
        Self {
            id,
            name,
            device_type,
            manufacturer,
            model,
            version: (1, 0, 0),
            capabilities: match device_type {
                DeviceType::Sensor => DeviceCapabilities::SENSOR_DEFAULT,
                DeviceType::Motor => DeviceCapabilities::MOTOR_DEFAULT,
                DeviceType::Communication => DeviceCapabilities::COMMUNICATION_DEFAULT,
                _ => DeviceCapabilities {
                    readable: true,
                    writable: true,
                    configurable: true,
                    interruptible: false,
                    real_time: false,
                    dma_capable: false,
                },
            },
            bus_type,
            bus_address,
            interrupt_line: None,
            dma_channel: None,
            sample_rate_hz: None,
            power_consumption_mw: 100,
        }
    }
}

/// Generic device interface trait
pub trait GenericDevice: Send + Sync {
    fn id(&self) -> DeviceId;
    fn metadata(&self) -> &DeviceMetadata;
    fn init(&mut self) -> AeroResult<()>;
    fn deinit(&mut self) -> AeroResult<()>;
    fn reset(&mut self) -> AeroResult<()>;
    fn status(&self) -> DeviceStatus;
    fn health(&self) -> HealthStatus;
    
    fn is_ready(&self) -> bool {
        matches!(self.status(), DeviceStatus::Ready | DeviceStatus::Running)
    }
    
    fn suspend(&mut self) -> AeroResult<()> { Ok(()) }
    fn resume(&mut self) -> AeroResult<()> { Ok(()) }
    fn read_property(&self, key: &str) -> AeroResult<u32>;
    fn write_property(&mut self, key: &str, value: u32) -> AeroResult<()>;
    fn read_raw(&mut self, buf: &mut [u8]) -> AeroResult<usize>;
    fn write_raw(&mut self, data: &[u8]) -> AeroResult<()>;
    
    fn get_stats(&self) -> DeviceStats {
        DeviceStats::default()
    }
}

/// Device statistics
#[derive(Debug, Clone, Copy, Default)]
pub struct DeviceStats {
    pub read_count: u32,
    pub write_count: u32,
    pub error_count: u32,
    pub last_read_ns: u64,
    pub last_error_ns: u64,
    pub uptime_ms: u32,
}

/// Device Event
#[derive(Debug, Clone, Copy)]
pub enum DeviceEvent {
    Initialized(DeviceId),
    Deinitialized(DeviceId),
    StatusChanged(DeviceId, DeviceStatus),
    HealthChanged(DeviceId, HealthStatus),
    Error(DeviceId),
    DataAvailable(DeviceId),
}

pub type DeviceEventCallback = fn(DeviceEvent) -> AeroResult<()>;

/// Central Device Registry
pub struct DeviceRegistry {
    devices: BTreeMap<DeviceId, Box<dyn GenericDevice>>,
    device_metadata: BTreeMap<DeviceId, DeviceMetadata>,
    next_id: u32,
    event_callbacks: [Option<DeviceEventCallback>; 8],
    callback_count: usize,
}

impl DeviceRegistry {
    pub fn new() -> Self {
        Self {
            devices: BTreeMap::new(),
            device_metadata: BTreeMap::new(),
            next_id: 1,
            event_callbacks: [None; 8],
            callback_count: 0,
        }
    }

    pub fn register(&mut self, mut device: Box<dyn GenericDevice>) -> AeroResult<DeviceId> {
        let id = device.id();
        device.init()?;
        let metadata = device.metadata().clone();
        self.device_metadata.insert(id, metadata);
        self.devices.insert(id, device);
        self.fire_event(DeviceEvent::Initialized(id));
        Ok(id)
    }

    pub fn unregister(&mut self, id: DeviceId) -> AeroResult<()> {
        if let Some(mut device) = self.devices.remove(&id) {
            device.deinit()?;
            self.device_metadata.remove(&id);
            self.fire_event(DeviceEvent::Deinitialized(id));
            Ok(())
        } else {
            Err(AeroError::HardwareNotFound)
        }
    }

    pub fn get(&self, id: DeviceId) -> AeroResult<&dyn GenericDevice> {
        self.devices.get(&id).map(|d| d.as_ref() as &dyn GenericDevice).ok_or(AeroError::HardwareNotFound)
    }

    pub fn get_mut(&mut self, id: DeviceId) -> AeroResult<&mut dyn GenericDevice> {
        self.devices.get_mut(&id).map(|d| d.as_mut() as &mut dyn GenericDevice).ok_or(AeroError::HardwareNotFound)
    }

    pub fn find_by_type(&self, device_type: DeviceType) -> Vec<DeviceId> {
        self.device_metadata.iter().filter(|(_, m)| m.device_type == device_type).map(|(id, _)| *id).collect()
    }

    pub fn find_by_bus(&self, bus_type: BusType) -> Vec<DeviceId> {
        self.device_metadata.iter().filter(|(_, m)| m.bus_type == bus_type).map(|(id, _)| *id).collect()
    }

    pub fn find_by_manufacturer(&self, manufacturer: &str) -> Vec<DeviceId> {
        self.device_metadata.iter().filter(|(_, m)| m.manufacturer.contains(manufacturer)).map(|(id, _)| *id).collect()
    }

    pub fn find_by_name(&self, pattern: &str) -> Vec<DeviceId> {
        self.device_metadata.iter().filter(|(_, m)| m.name.contains(pattern)).map(|(id, _)| *id).collect()
    }

    pub fn get_metadata(&self, id: DeviceId) -> AeroResult<&DeviceMetadata> {
        self.device_metadata.get(&id).ok_or(AeroError::HardwareNotFound)
    }

    pub fn count(&self) -> usize { self.devices.len() }
    pub fn count_by_type(&self, device_type: DeviceType) -> usize { self.find_by_type(device_type).len() }
    pub fn list_all(&self) -> Vec<DeviceId> { self.devices.keys().copied().collect() }

    pub fn health_check_all(&self) -> BTreeMap<DeviceId, HealthStatus> {
        self.devices.iter().map(|(id, d)| (*id, d.health())).collect()
    }

    pub fn health_check_type(&self, device_type: DeviceType) -> BTreeMap<DeviceId, HealthStatus> {
        self.find_by_type(device_type).iter().filter_map(|id| self.devices.get(id).map(|d| (*id, d.health()))).collect()
    }

    pub fn init_all(&mut self) -> AeroResult<()> {
        let ids: Vec<_> = self.devices.keys().copied().collect();
        for id in ids { self.get_mut(id)?.init()?; }
        Ok(())
    }

    pub fn deinit_all(&mut self) -> AeroResult<()> {
        let ids: Vec<_> = self.devices.keys().copied().collect();
        for id in ids { self.get_mut(id)?.deinit()?; }
        Ok(())
    }

    pub fn resume_all(&mut self) -> AeroResult<()> {
        let ids: Vec<_> = self.devices.keys().copied().collect();
        for id in ids { self.get_mut(id)?.resume()?; }
        Ok(())
    }

    pub fn suspend_all(&mut self) -> AeroResult<()> {
        let ids: Vec<_> = self.devices.keys().copied().collect();
        for id in ids { self.get_mut(id)?.suspend()?; }
        Ok(())
    }

    pub fn register_event_callback(&mut self, callback: DeviceEventCallback) -> AeroResult<()> {
        if self.callback_count >= 8 { return Err(AeroError::ResourceExhausted); }
        self.event_callbacks[self.callback_count] = Some(callback);
        self.callback_count += 1;
        Ok(())
    }

    fn fire_event(&self, event: DeviceEvent) {
        for i in 0..self.callback_count {
            if let Some(cb) = self.event_callbacks[i] { let _ = cb(event); }
        }
    }

    pub fn get_stats(&self, id: DeviceId) -> AeroResult<DeviceStats> {
        self.get(id).map(|d| d.get_stats())
    }

    pub fn reset_all(&mut self) -> AeroResult<()> {
        let ids: Vec<_> = self.devices.keys().copied().collect();
        for id in ids { self.get_mut(id)?.reset()?; }
        Ok(())
    }
}

impl Default for DeviceRegistry { fn default() -> Self { Self::new() } }
