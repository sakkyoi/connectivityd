use serde::{Deserialize, Serialize};

use super::ip::{Ipv4Config, Ipv6Config};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthernetConfig {
    pub interface_id: String,
    pub ipv4: Ipv4Config,
    pub ipv6: Ipv6Config,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyEthernetConfigRequest {
    pub interface_id: String,
    pub config: EthernetConfig,
}
