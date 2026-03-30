use std::sync::Arc;

use connectivity_app::context::AppContext;
use connectivity_domain::network::{
    ethernet::{ApplyEthernetConfigRequest, EthernetConfig},
    ip::{IpAssignment, Ipv4Config, Ipv6Address, Ipv6Config},
};
use serde::{Deserialize, Serialize};
use zbus::{fdo, interface, zvariant::Type};

use crate::error::map_domain_error;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Ipv4ConfigDto {
    pub assignment: String,
    pub address: String,
    pub prefix_len: u8,
    pub gateway: String,
    pub dns_servers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Ipv6AddressDto {
    pub address: String,
    pub prefix_len: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Ipv6ConfigDto {
    pub assignment: String,
    pub addresses: Vec<Ipv6AddressDto>,
    pub gateway: String,
    pub dns_servers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct EthernetConfigDto {
    pub interface_id: String,
    pub ipv4: Ipv4ConfigDto,
    pub ipv6: Ipv6ConfigDto, // Optional, determine by Ipv6ConfigDto.enabled
}

fn assignment_to_string(value: &IpAssignment) -> String {
    match value {
        IpAssignment::Dhcp => "dhcp",
        IpAssignment::Static => "static",
        IpAssignment::Auto => "auto",
    }
        .to_string()
}

impl From<Ipv4Config> for Ipv4ConfigDto {
    fn from(value: Ipv4Config) -> Self {
        Self {
            assignment: assignment_to_string(&value.assignment),
            address: value.address.unwrap_or_default(),
            prefix_len: value.prefix_len.unwrap_or_default(),
            gateway: value.gateway.unwrap_or_default(),
            dns_servers: value.dns_servers,
        }
    }
}

impl From<Ipv6Address> for Ipv6AddressDto {
    fn from(value: Ipv6Address) -> Self {
        Self {
            address: value.address,
            prefix_len: value.prefix_len,
        }
    }
}

impl From<Ipv6Config> for Ipv6ConfigDto {
    fn from(value: Ipv6Config) -> Self {
        Self {
            assignment: assignment_to_string(&value.assignment),
            addresses: value.addresses.into_iter().map(Into::into).collect(),
            gateway: value.gateway.unwrap_or_default(),
            dns_servers: value.dns_servers,
        }
    }
}

impl From<EthernetConfig> for EthernetConfigDto {
    fn from(value: EthernetConfig) -> Self {
        Self {
            interface_id: value.interface_id,
            ipv4: value.ipv4.into(),
            ipv6: value.ipv6.into(),
        }
    }
}

pub struct EthernetObject {
    ctx: Arc<AppContext>,
}

impl EthernetObject {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        Self { ctx }
    }
}

#[interface(name = "com.example.Connectivity.Network.Ethernet1")]
impl EthernetObject {
    async fn get_ethernet_config(&self, interface_id: String) -> fdo::Result<EthernetConfigDto> {
        let config = self
            .ctx
            .network
            .get_ethernet_config(&interface_id)
            .await
            .map_err(map_domain_error)?;

        Ok(config.into())
    }

    async fn apply_ethernet_config(&self, config: EthernetConfigDto) -> fdo::Result<()> {
        let request = ApplyEthernetConfigRequest {
            interface_id: config.interface_id.clone(),
            config: EthernetConfig {
                interface_id: config.interface_id,
                ipv4: Ipv4Config {
                    assignment: match config.ipv4.assignment.as_str() {
                        "dhcp" => IpAssignment::Dhcp,
                        "static" => IpAssignment::Static,
                        "auto" => IpAssignment::Auto,
                        _ => return Err(fdo::Error::InvalidArgs("invalid ipv4 assignment".into())),
                    },
                    address: if config.ipv4.address.is_empty() {
                        None
                    } else {
                        Some(config.ipv4.address)
                    },
                    prefix_len: Some(config.ipv4.prefix_len),
                    gateway: if config.ipv4.gateway.is_empty() {
                        None
                    } else {
                        Some(config.ipv4.gateway)
                    },
                    dns_servers: config.ipv4.dns_servers,
                },
                ipv6: Ipv6Config {
                    assignment: match config.ipv6.assignment.as_str() {
                        "dhcp" => IpAssignment::Dhcp,
                        "static" => IpAssignment::Static,
                        "auto" => IpAssignment::Auto,
                        _ => return Err(fdo::Error::InvalidArgs("invalid ipv6 assignment".into())),
                    },
                    addresses: config.ipv6
                        .addresses
                        .into_iter()
                        .map(|a| Ipv6Address {
                            address: a.address,
                            prefix_len: a.prefix_len,
                        })
                        .collect(),
                    gateway: if config.ipv6.gateway.is_empty() {
                        None
                    } else {
                        Some(config.ipv6.gateway)
                    },
                    dns_servers: config.ipv6.dns_servers,
                },
            }
        };

        self.ctx
            .network
            .apply_ethernet_config(request)
            .await
            .map_err(map_domain_error)
    }
}
