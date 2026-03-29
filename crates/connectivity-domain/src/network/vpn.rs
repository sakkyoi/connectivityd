use serde::{Deserialize, Serialize};

use super::common::ConnectionState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VpnKind {
    WireGuard,
    OpenVpn,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnProfile {
    pub id: String,
    pub kind: VpnKind,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectVpnRequest {
    pub profile_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnStatus {
    pub profile_id: String,
    pub state: ConnectionState,
}
