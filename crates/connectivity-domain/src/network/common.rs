use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkInterfaceKind {
    Ethernet,
    Wifi,
    Vpn,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterfaceState {
    Down,
    Up,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Failed,
}
