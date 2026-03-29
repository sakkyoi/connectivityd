use serde::{Deserialize, Serialize};

use super::ip::Ipv4Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiVisibleNetwork {
    pub id: String,
    pub ssid: String,
    pub signal_strength: i16,
    pub security: String,
    pub connected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiScanRequest {
    pub interface_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiConnectRequest {
    pub interface_id: String,
    pub ssid: String,
    pub passphrase: String,
    pub ipv4: Option<Ipv4Config>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedWifiProfile {
    pub id: String,
    pub interface_id: Option<String>,
    pub ssid: String,
}