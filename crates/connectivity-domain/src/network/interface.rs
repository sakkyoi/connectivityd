use serde::{Deserialize, Serialize};

use super::common::{NetworkInterfaceKind, OperationalState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub id: String,
    pub name: String,
    pub kind: NetworkInterfaceKind,
    pub enabled: bool,
    pub carrier: Option<bool>,
    pub operational_state: OperationalState,
}
