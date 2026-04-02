use connectivity_domain::network::{
    common::{InterfaceState, NetworkInterfaceKind},
    interface::NetworkInterface,
    wifi::{WifiNetwork, WifiSecurity},
};
use nmrs::{DeviceType, DeviceState, Device, Network};


pub fn map_device<T>(device: T) -> NetworkInterface
where
    T: IntoNmrsDeviceLike,
{
    let d = device.into_nmrs_like();

    NetworkInterface {
        id: d.interface.clone(),
        name: d.interface,
        kind: map_device_kind(&d.device_type),
        enabled: true,
        state: map_interface_state(&d.state),
        carrier: None,
        mac_address: None,
    }
}

pub fn map_visible_wifi<T>(network: T, current_ssid: Option<&str>) -> WifiNetwork
where
    T: IntoNmrsWifiLike,
{
    let n = network.into_wifi_like();

    WifiNetwork {
        id: n.ssid.clone(),
        ssid: n.ssid.clone(),
        signal_strength: n.strength.unwrap_or(0),
        security: WifiSecurity::Unknown,
        connected: current_ssid == Some(n.ssid.as_str()),
        saved: false,
    }
}

/// TODO
fn map_device_kind(value: &str) -> NetworkInterfaceKind {
    match value.to_ascii_lowercase().as_str() {
        _ => NetworkInterfaceKind::Unknown,
    }
}

/// TODO
fn map_interface_state(value: &str) -> InterfaceState {
    match value.to_ascii_lowercase().as_str() {
        _ => InterfaceState::Unknown,
    }
}

pub struct DeviceLike {
    pub interface: String,
    pub device_type: String,
    pub state: String,
}

pub trait IntoNmrsDeviceLike {
    fn into_nmrs_like(self) -> DeviceLike;
}

pub struct WifiLike {
    pub ssid: String,
    pub strength: Option<u8>,
}

pub trait IntoNmrsWifiLike {
    fn into_wifi_like(self) -> WifiLike;
}

impl IntoNmrsDeviceLike for Device {
    fn into_nmrs_like(self) -> DeviceLike {
        DeviceLike {
            interface: self.interface,
            device_type: self.device_type.to_string(),
            state: self.state.to_string(),
        }
    }
}

impl IntoNmrsWifiLike for Network {
    fn into_wifi_like(self) -> WifiLike {
        WifiLike {
            ssid: self.ssid,
            strength: self.strength,
        }
    }
}
