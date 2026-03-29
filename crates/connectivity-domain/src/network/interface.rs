use serde::{Deserialize, Serialize};

use super::common::{InterfaceState, NetworkInterfaceKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub id: String,
    pub name: String,
    pub kind: NetworkInterfaceKind,
    pub enabled: bool,
    pub state: InterfaceState,
    pub carrier: Option<bool>,
    pub mac_address: Option<String>,
}
