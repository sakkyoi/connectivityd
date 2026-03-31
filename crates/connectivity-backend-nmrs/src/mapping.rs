use connectivity_domain::network::{
    common::{InterfaceState, NetworkInterfaceKind},
    wifi::WifiSecurity,
};
use nmrs::{DeviceType, DeviceState};

pub fn map_device_type(value: DeviceType) -> NetworkInterfaceKind {
    match value {
        DeviceType::Wifi => NetworkInterfaceKind::Wifi,
        DeviceType::Ethernet => NetworkInterfaceKind::Ethernet,
        _ => NetworkInterfaceKind::Unknown,
    }
}

pub fn map_device_state(value: DeviceState) -> InterfaceState {
    match value {
        DeviceState::Activated => InterfaceState::Up,
        _ => InterfaceState::Unknown,
    }
}