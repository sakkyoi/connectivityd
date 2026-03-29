use async_trait::async_trait;
use connectivity_domain::{
    network::{
        ethernet::{ApplyEthernetConfigRequest, EthernetConfig},
        interface::NetworkInterface,
        vpn::{ConnectVpnRequest, VpnProfile},
        wifi::{SavedWifiProfile, WifiConnectRequest, WifiScanRequest, WifiVisibleNetwork},
    },
    ConnectivityError,
};

#[async_trait]
pub trait NetworkBackend: Send + Sync {
    async fn list_interfaces(&self) -> Result<Vec<NetworkInterface>, ConnectivityError>;

    async fn set_interface_enabled(
        &self,
        interface_id: &str,
        enabled: bool,
    ) -> Result<(), ConnectivityError>;

    async fn get_ethernet_config(
        &self,
        interface_id: &str,
    ) -> Result<EthernetConfig, ConnectivityError>;

    async fn apply_ethernet_config(
        &self,
        request: ApplyEthernetConfigRequest,
    ) -> Result<(), ConnectivityError>;

    async fn scan_wifi(&self, request: WifiScanRequest) -> Result<(), ConnectivityError>;

    async fn list_visible_wifi_networks(
        &self,
        interface_id: Option<&str>,
    ) -> Result<Vec<WifiVisibleNetwork>, ConnectivityError>;

    async fn connect_wifi(&self, request: WifiConnectRequest) -> Result<(), ConnectivityError>;

    async fn disconnect_wifi(
        &self,
        interface_id: Option<&str>,
    ) -> Result<(), ConnectivityError>;

    async fn list_saved_wifi_profiles(
        &self,
        interface_id: Option<&str>,
    ) -> Result<Vec<SavedWifiProfile>, ConnectivityError>;

    async fn forget_wifi_profile(&self, profile_id: &str) -> Result<(), ConnectivityError>;

    async fn list_vpn_profiles(&self) -> Result<Vec<VpnProfile>, ConnectivityError>;

    async fn connect_vpn(&self, request: ConnectVpnRequest) -> Result<(), ConnectivityError>;

    async fn disconnect_vpn(&self, profile_id: &str) -> Result<(), ConnectivityError>;
}
