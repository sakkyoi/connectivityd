use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkInterfaceKind {
    Ethernet,
    Wifi,
    Vpn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationalState {
    Unknown,
    Down,
    Up,
    Dormant,
}
