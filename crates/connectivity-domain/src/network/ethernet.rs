use serde::{Deserialize, Serialize};

use super::ip::Ipv4Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthernetConfig {
    pub interface_id: String,
    pub ipv4: Ipv4Config,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyEthernetConfigRequest {
    pub interface_id: String,
    pub config: EthernetConfig,
}
