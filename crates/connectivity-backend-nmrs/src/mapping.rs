use connectivity_domain::network::{
    common::{InterfaceState, NetworkInterfaceKind},
    interface::NetworkInterface,
    wifi::WifiSecurity,
};
use nmrs::{DeviceType, DeviceState};

pub fn map_interface_kind(device_type: DeviceType) -> NetworkInterfaceKind {
    match device_type {
        DeviceType::Wifi => NetworkInterfaceKind::Wifi,
        DeviceType::Ethernet => NetworkInterfaceKind::Ethernet,
        _ => NetworkInterfaceKind::Unknown,
    }
}

pub fn map_interface_state(state: DeviceState) -> InterfaceState {
    match state {
        DeviceState::Activated => InterfaceState::Up,
        DeviceState::Other(_) => InterfaceState::Unknown,
        _ => InterfaceState::Down,
    }
}

pub fn map_device(
    interface: String,
    device_type: DeviceType,
    state: DeviceState,
) -> NetworkInterface {
    NetworkInterface {
        id: interface.clone(),
        name: interface,
        kind: map_interface_kind(device_type),
        enabled: true,
        state: map_interface_state(state),
        carrier: None,
        mac_address: None,
    }
}
