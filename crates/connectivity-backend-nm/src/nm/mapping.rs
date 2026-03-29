use connectivity_domain::network::{
    common::{InterfaceState, NetworkInterfaceKind},
    wifi::WifiSecurity,
};

pub const NM_DEVICE_TYPE_ETHERNET: u32 = 1;
pub const NM_DEVICE_TYPE_WIFI: u32 = 2;
pub const NM_DEVICE_TYPE_WIREGUARD: u32 = 29;

pub fn map_device_type(value: u32) -> NetworkInterfaceKind {
    match value {
        NM_DEVICE_TYPE_ETHERNET => NetworkInterfaceKind::Ethernet,
        NM_DEVICE_TYPE_WIFI => NetworkInterfaceKind::Wifi,
        NM_DEVICE_TYPE_WIREGUARD => NetworkInterfaceKind::Vpn,
        _ => NetworkInterfaceKind::Unknown,
    }
}

pub fn map_device_state_simple(value: u32) -> InterfaceState {
    match value {
        100 => InterfaceState::Up,
        30 | 40 | 50 | 60 | 70 | 80 | 90 => InterfaceState::Up,
        10 | 20 => InterfaceState::Down,
        _ => InterfaceState::Unknown,
    }
}

pub fn map_wifi_security(flags: u32, wpa_flags: u32, rsn_flags: u32) -> WifiSecurity {
    let privacy = (flags & 0x1) != 0;
    let has_wpa = wpa_flags != 0;
    let has_rsn = rsn_flags != 0;

    match (privacy, has_wpa, has_rsn) {
        (false, false, false) => WifiSecurity::Open,
        (true, false, false) => WifiSecurity::Wep,
        (_, true, false) => WifiSecurity::Wpa,
        (_, false, true) => WifiSecurity::Wpa2,
        (_, true, true) => WifiSecurity::Wpa2, // conservative, need to categorize into WPA3/SAE
    }
}
