use std::sync::Arc;

use connectivity_app::context::AppContext;
use connectivity_domain::network::{
    common::NetworkInterfaceKind,
    interface::NetworkInterface,
    wifi::{WifiConnectRequest, WifiNetwork, SavedWifiNetwork, WifiScanRequest, WifiSecurity},
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
}

impl From<SavedWifiNetwork> for SavedWifiNetworkDto {
    fn from(value: SavedWifiNetwork) -> Self {
        Self {
            id: value.id,
            ssid: value.ssid,
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

    async fn set_interface_enabled(&self, interface_id: String, enabled: bool) -> fdo::Result<()> {
        self.ctx
            .network
            .set_interface_enabled(&interface_id, enabled)
            .await
            .map_err(map_domain_error)
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
    async fn scan_wifi(&self, interface_id: Optional<String>) -> fdo::Result<()> {
        let interface_id: Option<String> = interface_id.into();
        self.ctx
            .network
            .scan_wifi(WifiScanRequest { interface_id })
            .await
            .map_err(map_domain_error)
    }

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
