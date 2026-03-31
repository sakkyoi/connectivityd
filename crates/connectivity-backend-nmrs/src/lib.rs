use async_trait::async_trait;
use connectivity_backend::network::NetworkBackend;
use connectivity_domain::{
    network::{
        ethernet::{ApplyEthernetConfigRequest, EthernetConfig},
        interface::{NetworkInterface},
        vpn::{ConnectVpnRequest, VpnProfile, VpnStatus},
        wifi::{SavedWifiNetwork, WifiConnectRequest, WifiNetwork, WifiScanRequest},
    },
    ConnectivityError,
};
use nmrs::NetworkManager;

pub fn map_zbus_err(e: ConnectivityError) -> ConnectivityError {
    ConnectivityError::BackendFailure(e.to_string())
}

pub struct NetworkManagerBackend;

impl NetworkManagerBackend {
    pub fn new() -> Self {
        Self
    }

    pub async fn manager(&self) -> Result<NetworkManager, ConnectivityError> {
        NetworkManager::new()
            .await
            .map_err(|e| ConnectivityError::BackendFailure(e.to_string()))
    }
}

#[async_trait]
impl NetworkBackend for NetworkManagerBackend {
    async fn list_interfaces(&self) -> Result<Vec<NetworkInterface>, ConnectivityError> {
        // let manager = self
        //     .manager()
        //     .await?;
        Err(ConnectivityError::Unsupported)
    }

    async fn get_interface(&self, interface_id: &str) -> Result<NetworkInterface, ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn set_interface_enabled(&self, interface_id: &str, enabled: bool) -> Result<(), ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn get_ethernet_config(&self, interface_id: &str) -> Result<EthernetConfig, ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn apply_ethernet_config(&self, request: ApplyEthernetConfigRequest) -> Result<(), ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn scan_wifi(&self, request: WifiScanRequest) -> Result<(), ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn list_visible_wifi_networks(&self, interface_id: Option<&str>) -> Result<Vec<WifiNetwork>, ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn connect_wifi(&self, request: WifiConnectRequest) -> Result<(), ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn disconnect_wifi(&self, interface_id: Option<&str>) -> Result<(), ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn list_saved_wifi_networks(&self, interface_id: Option<&str>) -> Result<Vec<SavedWifiNetwork>, ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn forget_wifi_network(&self, network_id: &str) -> Result<(), ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn list_vpn_profiles(&self) -> Result<Vec<VpnProfile>, ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn connect_vpn(&self, request: ConnectVpnRequest) -> Result<(), ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn disconnect_vpn(&self, profile_id: &str) -> Result<(), ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn get_vpn_status(&self, profile_id: &str) -> Result<VpnStatus, ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }
}
