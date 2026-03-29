use async_trait::async_trait;
use connectivity_backend::network::NetworkBackend;
use connectivity_domain::{
    network::{
        ethernet::{ApplyEthernetConfigRequest, EthernetConfig},
        interface::NetworkInterface,
        vpn::{ConnectVpnRequest, VpnProfile},
        wifi::{SavedWifiProfile, WifiConnectRequest, WifiScanRequest, WifiVisibleNetwork},
    },
    ConnectivityError,
};

pub struct NetworkManagerBackend;

impl NetworkManagerBackend {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl NetworkBackend for NetworkManagerBackend {
    async fn list_interfaces(&self) -> Result<Vec<NetworkInterface>, ConnectivityError> {
        Ok(vec![])
    }

    async fn set_interface_enabled(
        &self,
        _interface_id: &str,
        _enabled: bool,
    ) -> Result<(), ConnectivityError> {
        Ok(())
    }

    async fn get_ethernet_config(
        &self,
        _interface_id: &str,
    ) -> Result<EthernetConfig, ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn apply_ethernet_config(
        &self,
        _request: ApplyEthernetConfigRequest,
    ) -> Result<(), ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn scan_wifi(
        &self,
        _request: WifiScanRequest,
    ) -> Result<(), ConnectivityError> {
        Ok(())
    }

    async fn list_visible_wifi_networks(
        &self,
        _interface_id: Option<&str>,
    ) -> Result<Vec<WifiVisibleNetwork>, ConnectivityError> {
        Ok(vec![])
    }

    async fn connect_wifi(
        &self,
        _request: WifiConnectRequest,
    ) -> Result<(), ConnectivityError> {
        Ok(())
    }

    async fn disconnect_wifi(
        &self,
        _interface_id: Option<&str>,
    ) -> Result<(), ConnectivityError> {
        Ok(())
    }

    async fn list_saved_wifi_profiles(
        &self,
        _interface_id: Option<&str>,
    ) -> Result<Vec<SavedWifiProfile>, ConnectivityError> {
        Ok(vec![])
    }

    async fn forget_wifi_profile(&self, _profile_id: &str) -> Result<(), ConnectivityError> {
        Ok(())
    }

    async fn list_vpn_profiles(&self) -> Result<Vec<VpnProfile>, ConnectivityError> {
        Ok(vec![])
    }

    async fn connect_vpn(&self, _request: ConnectVpnRequest) -> Result<(), ConnectivityError> {
        Ok(())
    }

    async fn disconnect_vpn(&self, _profile_id: &str) -> Result<(), ConnectivityError> {
        Ok(())
    }
}
