mod nm;

use async_trait::async_trait;
use connectivity_backend::network::NetworkBackend;
use connectivity_domain::{
    network::{
        interface::NetworkInterface,
        ip::{Ipv4Config, Ipv6Config},
        vpn::{ConnectVpnRequest, VpnProfile, VpnStatus},
        wifi::{SavedWifiNetwork, WifiConnectRequest, WifiNetwork},
    },
    ConnectivityError,
};
use zbus::{zvariant::OwnedObjectPath, Connection};

pub fn map_zbus_err(e: zbus::Error) -> ConnectivityError {
    ConnectivityError::BackendFailure(e.to_string())
}

pub struct NetworkManagerBackend;

impl NetworkManagerBackend {
    pub fn new() -> Self {
        Self
    }

    pub async fn nm(&self) -> Result<Connection, ConnectivityError> {
        Connection::system()
            .await
            .map_err(map_zbus_err)
    }
}

#[async_trait]
impl NetworkBackend for NetworkManagerBackend {
    //
    // Interface inventory
    //

    async fn list_interfaces(&self) -> Result<Vec<NetworkInterface>, ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn get_interface(&self, _interface_id: &str) -> Result<NetworkInterface, ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    //
    // Wi-Fi control
    //

    async fn set_wifi_enabled(&self, _interface_id: Option<&str>, _enabled: bool) -> Result<(), ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn list_visible_wifi_networks(&self, _interface_id: Option<&str>) -> Result<Vec<WifiNetwork>, ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn connect_wifi(&self, _request: WifiConnectRequest) -> Result<(), ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn disconnect_wifi(&self, _interface_id: Option<&str>) -> Result<(), ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn list_saved_wifi_networks(&self, _interface_id: Option<&str>) -> Result<Vec<SavedWifiNetwork>, ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn forget_wifi_network(&self, _network_id: &str) -> Result<(), ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    //
    // IP config
    //

    async fn get_ipv4_config(&self, _interface_id: &str) -> Result<Option<Ipv4Config>, ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn set_ipv4_config(&self, _interface_id: &str, _config: Ipv4Config) -> Result<(), ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn get_ipv6_config(&self, _interface_id: &str) -> Result<Option<Ipv6Config>, ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn set_ipv6_config(&self, _interface_id: &str, _config: Ipv6Config) -> Result<(), ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    //
    // VPN
    //

    async fn list_vpn_profiles(&self) -> Result<Vec<VpnProfile>, ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn connect_vpn(&self, _request: ConnectVpnRequest) -> Result<(), ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn disconnect_vpn(&self, _profile_id: &str) -> Result<(), ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn get_vpn_status(&self, _profile_id: &str) -> Result<VpnStatus, ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }
}
