use serde::{Deserialize, Serialize};
use nmrs::{DeviceType, DeviceState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkInterfaceKind {
    Ethernet,
    Wifi,
    Vpn,
    Unknown,
}

impl From<DeviceType> for NetworkInterfaceKind {
    fn from(value: DeviceType) -> Self {
        match value {
            DeviceType::Wifi => Self::Wifi,
            DeviceType::Ethernet => Self::Ethernet,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterfaceState {
    Down,
    Up,
    Unknown,
}

impl From<DeviceState> for InterfaceState {
    fn from(value: DeviceState) -> Self {
        match value {
            DeviceState::Activated => InterfaceState::Up,
            DeviceState::Other(_) => InterfaceState::Unknown,
            _ => InterfaceState::Down,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Failed,
}
