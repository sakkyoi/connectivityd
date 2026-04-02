use std::sync::Arc;

use connectivity_app::context::AppContext;
use connectivity_domain::network::{
    common::{NetworkInterfaceKind, ConnectionState},
    interface::NetworkInterface,
    ip::{IpAssignment, Ipv4Config, Ipv6Address, Ipv6Config},
    vpn::{ConnectVpnRequest, VpnKind, VpnProfile, VpnStatus},
    wifi::{SavedWifiNetwork, WifiConnectRequest, WifiNetwork, WifiSecurity},
};
use serde::{Deserialize, Serialize};
use zbus::{fdo, interface, zvariant::{Type, Optional}};

use crate::error::map_domain_error;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct NetworkInterfaceDto {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub enabled: bool,
    pub state: String,
    pub carrier: bool,
    pub mac_address: String,
}

impl From<NetworkInterface> for NetworkInterfaceDto {
    fn from(value: NetworkInterface) -> Self {
        let kind = match value.kind {
            NetworkInterfaceKind::Ethernet => "ethernet",
            NetworkInterfaceKind::Wifi => "wifi",
            NetworkInterfaceKind::Vpn => "vpn",
            NetworkInterfaceKind::Unknown => "unknown",
        }
            .to_string();

        Self {
            id: value.id,
            name: value.name,
            kind,
            enabled: value.enabled,
            state: format!("{:?}", value.state).to_lowercase(),
            carrier: value.carrier.unwrap_or(false),
            mac_address: value.mac_address.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct WifiNetworkDto {
    pub id: String,
    pub ssid: String,
    pub signal_strength: u8,
    pub security: String,
    pub connected: bool,
    pub saved: bool,
}

impl From<WifiNetwork> for WifiNetworkDto {
    fn from(value: WifiNetwork) -> Self {
        let security = match value.security {
            WifiSecurity::Open => "open",
            WifiSecurity::Wep => "wep",
            WifiSecurity::Wpa => "wpa",
            WifiSecurity::Wpa2 => "wpa2",
            WifiSecurity::Wpa3 => "wpa3",
            WifiSecurity::Unknown => "unknown",
        }
            .to_string();

        Self {
            id: value.id,
            ssid: value.ssid,
            signal_strength: value.signal_strength,
            security,
            connected: value.connected,
            saved: value.saved,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct SavedWifiNetworkDto {
    pub id: String,
    pub ssid: String,
    pub connection_path: String,
}

impl From<SavedWifiNetwork> for SavedWifiNetworkDto {
    fn from(value: SavedWifiNetwork) -> Self {
        Self {
            id: value.id,
            ssid: value.ssid,
            connection_path: value.connection_path.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
pub struct Ipv6ConfigDto {
    pub assignment: String,
    pub addresses: Vec<Ipv6AddressDto>,
    pub gateway: String,
    pub dns_servers: Vec<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct VpnProfileDto {
    pub id: String,
    pub kind: String,
    pub display_name: String,
}

impl From<VpnProfile> for VpnProfileDto {
    fn from(value: VpnProfile) -> Self {
        let kind = match value.kind {
            VpnKind::WireGuard => "wireguard",
            VpnKind::OpenVpn => "openvpn",
            VpnKind::Unknown => "unknown",
        }
            .to_string();

        Self {
            id: value.id,
            kind,
            display_name: value.display_name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct VpnStatusDto {
    pub profile_id: String,
    pub state: String,
}

impl From<VpnStatus> for VpnStatusDto {
    fn from(value: VpnStatus) -> Self {
        let state = match value.state {
            ConnectionState::Disconnected => "disconnected",
            ConnectionState::Connecting => "connecting",
            ConnectionState::Connected => "connected",
            ConnectionState::Failed => "failed",
        }
            .to_string();

        Self {
            profile_id: value.profile_id,
            state,
        }
    }
}

pub struct NetworkObject {
    ctx: Arc<AppContext>,
}

impl NetworkObject {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        Self { ctx }
    }
}

#[interface(name = "com.example.Connectivity.Network1")]
impl NetworkObject {
    async fn list_interfaces(&self) -> fdo::Result<Vec<NetworkInterfaceDto>> {
        let interfaces = self
            .ctx
            .network
            .list_interfaces()
            .await
            .map_err(map_domain_error)?;

        Ok(interfaces.into_iter().map(Into::into).collect())
    }

    async fn get_interface(&self, interface_id: String) -> fdo::Result<NetworkInterfaceDto> {
        let iface = self
            .ctx
            .network
            .get_interface(&interface_id)
            .await
            .map_err(map_domain_error)?;

        Ok(iface.into())
    }
}

pub struct WifiObject {
    ctx: Arc<AppContext>,
}

impl WifiObject {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        Self { ctx }
    }
}

#[interface(name = "com.example.Connectivity.Network.WiFi1")]
impl WifiObject {
    async fn list_visible_wifi_networks(
        &self,
        interface_id: Optional<String>,
    ) -> fdo::Result<Vec<WifiNetworkDto>> {
        let networks = self
            .ctx
            .network
            .list_visible_wifi_networks(interface_id.as_deref())
            .await
            .map_err(map_domain_error)?;

        Ok(networks.into_iter().map(Into::into).collect())
    }

    async fn connect_wifi(
        &self,
        interface_id: Optional<String>,
        ssid: String,
        passphrase: Optional<String>,
    ) -> fdo::Result<()> {
        self.ctx
            .network
            .connect_wifi(WifiConnectRequest {
                interface_id: interface_id.into(),
                ssid,
                passphrase: passphrase.into(),
                ipv4: None,
                ipv6: None,
            })
            .await
            .map_err(map_domain_error)
    }

    async fn disconnect_wifi(&self, interface_id: Optional<String>) -> fdo::Result<()> {
        self.ctx
            .network
            .disconnect_wifi(interface_id.as_deref())
            .await
            .map_err(map_domain_error)
    }

    async fn list_saved_wifi_networks(
        &self,
        interface_id: Optional<String>,
    ) -> fdo::Result<Vec<SavedWifiNetworkDto>> {
        let networks = self
            .ctx
            .network
            .list_saved_wifi_profiles(interface_id.as_deref())
            .await
            .map_err(map_domain_error)?;

        Ok(networks.into_iter().map(Into::into).collect())
    }

    async fn forget_wifi_network(&self, network_id: &str) -> fdo::Result<()> {
        self.ctx
            .network
            .forget_wifi_network(&network_id)
            .await
            .map_err(map_domain_error)
    }
}

pub struct IpObject {
    ctx: Arc<AppContext>,
}

impl IpObject {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        Self { ctx }
    }
}

#[interface(name = "com.example.Connectivity.Network.IP1")]
impl IpObject {
    async fn get_ipv4_config(&self, interface_id: String) -> fdo::Result<Optional<Ipv4ConfigDto>> {
        let config = self
            .ctx
            .network
            .get_ipv4_config(&interface_id)
            .await
            .map_err(map_domain_error)?;

        Ok(config.map(Into::into).into())
    }

    async fn set_ipv4_config(&self, interface_id: String, config: Ipv4ConfigDto) -> fdo::Result<()> {
        let config = Ipv4Config {
            assignment: match config.assignment.as_str() {
                "dhcp" => IpAssignment::Dhcp,
                "static" => IpAssignment::Static,
                "auto" => IpAssignment::Auto,
                _ => return Err(fdo::Error::InvalidArgs("invalid ipv4 assignment".into())),
            },
            address: if config.address.is_empty() {
                None
            } else {
                Some(config.address)
            },
            prefix_len: Some(config.prefix_len),
            gateway: if config.gateway.is_empty() {
                None
            } else {
                Some(config.gateway)
            },
            dns_servers:  config.dns_servers,
        };

        self
            .ctx
            .network
            .set_ipv4_config(&interface_id, config)
            .await
            .map_err(map_domain_error)
    }

    async fn get_ipv6_config(&self, interface_id: String) -> fdo::Result<Optional<Ipv6ConfigDto>> {
        let config = self
            .ctx
            .network
            .get_ipv6_config(&interface_id)
            .await
            .map_err(map_domain_error)?;

        Ok(config.map(Into::into).into())
    }

    async fn set_ipv6_config(&self, interface_id: String, config: Ipv6ConfigDto) -> fdo::Result<()> {
        let config = Ipv6Config {
            assignment: match config.assignment.as_str() {
                "dhcp" => IpAssignment::Dhcp,
                "static" => IpAssignment::Static,
                "auto" => IpAssignment::Auto,
                _ => return Err(fdo::Error::InvalidArgs("invalid ipv6 assignment".into())),
            },
            addresses: config
                .addresses
                .into_iter()
                .map(|a| Ipv6Address {
                    address: a.address,
                    prefix_len: a.prefix_len,
                })
                .collect(),
            gateway: if config.gateway.is_empty() {
                None
            } else {
                Some(config.gateway)
            },
            dns_servers: config.dns_servers,
        };

        self
            .ctx
            .network
            .set_ipv6_config(&interface_id, config)
            .await
            .map_err(map_domain_error)
    }
}

pub struct VpnObject {
    ctx: Arc<AppContext>,
}

impl VpnObject {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        Self { ctx }
    }
}

#[interface(name = "com.example.Connectivity.Network.VPN1")]
impl VpnObject {
    async fn list_vpn_profiles(&self) -> fdo::Result<Vec<VpnProfileDto>> {
        let profiles = self
            .ctx
            .network
            .list_vpn_profiles()
            .await
            .map_err(map_domain_error)?;

        Ok(profiles.into_iter().map(Into::into).collect())
    }

    async fn connect_vpn(&self, profile_id: String) -> fdo::Result<()> {
        self.ctx
            .network
            .connect_vpn(ConnectVpnRequest { profile_id })
            .await
            .map_err(map_domain_error)
    }

    async fn disconnect_vpn(&self, profile_id: String) -> fdo::Result<()> {
        self.ctx
            .network
            .disconnect_vpn(&profile_id)
            .await
            .map_err(map_domain_error)
    }

    async fn get_vpn_status(&self, profile_id: String) -> fdo::Result<VpnStatusDto> {
        let status = self
            .ctx
            .network
            .get_vpn_status(&profile_id)
            .await
            .map_err(map_domain_error)?;

        Ok(status.into())
    }
}
