mod mapping;

use std::thread::current;
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
use nmrs::{NetworkManager, ConnectionError};

use crate::mapping::map_device;

pub fn map_zbus_err(e: ConnectionError) -> ConnectivityError {
    ConnectivityError::BackendFailure(e.to_string())
}

pub struct NetworkManagerBackend;

impl NetworkManagerBackend {
    pub fn new() -> Self {
        Self
    }

    pub async fn nm(&self) -> Result<NetworkManager, ConnectivityError> {
        NetworkManager::new()
            .await
            .map_err(map_zbus_err)
    }
}

#[async_trait]
impl NetworkBackend for NetworkManagerBackend {
    async fn list_interfaces(&self) -> Result<Vec<NetworkInterface>, ConnectivityError> {
        let nm = self.nm().await?;
        let devices = nm
            .list_devices()
            .await
            .map_err(map_zbus_err)?;

        let result = devices
            .into_iter()
            .map(|d| map_device(d.interface, d.device_type, d.state))
            .collect();

        Ok(result)
    }

    async fn get_interface(&self, interface_id: &str) -> Result<NetworkInterface, ConnectivityError> {
        let interfaces = self.list_interfaces().await?;
        interfaces
            .into_iter()
            .find(|i| i.id == interface_id)
            .ok_or(ConnectivityError::InterfaceNotFound)
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
