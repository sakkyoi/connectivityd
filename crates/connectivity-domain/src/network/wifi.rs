use serde::{Deserialize, Serialize};

use super::ip::{Ipv4Config, Ipv6Config};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WifiSecurity {
    Open,
    Wep,
    Wpa,
    Wpa2,
    Wpa3,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiNetwork {
    pub id: String,
    pub ssid: String,
    pub signal_strength: u8,
    pub security: WifiSecurity,
    pub connected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiScanRequest {
    pub interface_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiConnectRequest {
    pub interface_id: Option<String>,
    pub ssid: String,
    pub passphrase: String,
    pub ipv4: Option<Ipv4Config>,
    pub ipv6: Option<Ipv6Config>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedWifiNetwork {
    pub id: String,
    pub ssid: String,
}
