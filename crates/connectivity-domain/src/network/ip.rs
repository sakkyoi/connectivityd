use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpAssignment {
    Dhcp,
    Static,
    Auto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ipv4Config {
    pub assignment: IpAssignment,
    pub address: Option<String>,
    pub prefix_len: Option<u8>,
    pub gateway: Option<String>,
    pub dns_servers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ipv6Address {
    pub address: String,
    pub prefix_len: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ipv6Config {
    pub assignment: IpAssignment,
    pub addresses: Vec<Ipv6Address>,
    pub gateway: Option<String>,
    pub dns_servers: Vec<String>,
}
