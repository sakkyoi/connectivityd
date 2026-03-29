use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpAssignment {
    Dhcp,
    Static,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ipv4Config {
    pub assignment: IpAssignment,
    pub address: Option<String>,
    pub prefix_len: Option<u8>,
    pub gateway: Option<String>,
    pub dns_servers: Vec<String>,
}
