use async_trait::async_trait;
use connectivity_domain::{
    network::{
        interface::NetworkInterface,
        ip::{Ipv4Config, Ipv6Config},
        vpn::{ConnectVpnRequest, VpnProfile, VpnStatus},
        wifi::{SavedWifiNetwork, WifiConnectRequest, WifiNetwork},
    },
    ConnectivityError,
};

#[async_trait]
pub trait NetworkBackend: Send + Sync {
    //
    // Interface inventory
    //

    async fn list_interfaces(&self) -> Result<Vec<NetworkInterface>, ConnectivityError>;

    async fn get_interface(&self, interface_id: &str) -> Result<NetworkInterface, ConnectivityError>;

    //
    // Wi-Fi
    //

    async fn set_wifi_enabled(
        &self,
        interface_id: Option<&str>,
        enabled: bool,
    ) -> Result<(), ConnectivityError>;

    async fn list_visible_wifi_networks(
        &self,
        interface_id: Option<&str>,
    ) -> Result<Vec<WifiNetwork>, ConnectivityError>;

    async fn connect_wifi(&self, request: WifiConnectRequest) -> Result<(), ConnectivityError>;

    async fn disconnect_wifi(
        &self,
        interface_id: Option<&str>,
    ) -> Result<(), ConnectivityError>;

    async fn list_saved_wifi_networks(
        &self,
        interface_id: Option<&str>,
    ) -> Result<Vec<SavedWifiNetwork>, ConnectivityError>;

    async fn forget_wifi_network(&self, network_id: &str) -> Result<(), ConnectivityError>;

    //
    // IP config
    //

    async fn get_ipv4_config(&self, interface_id: &str) -> Result<Option<Ipv4Config>, ConnectivityError>;

    async fn set_ipv4_config(
        &self,
        interface_id: &str,
        config: Ipv4Config,
    ) -> Result<(), ConnectivityError>;

    async fn get_ipv6_config(&self, interface_id: &str) -> Result<Option<Ipv6Config>, ConnectivityError>;

    async fn set_ipv6_config(
        &self,
        interface_id: &str,
        config: Ipv6Config,
    ) -> Result<(), ConnectivityError>;

    //
    // VPN
    //

    async fn list_vpn_profiles(&self) -> Result<Vec<VpnProfile>, ConnectivityError>;

    async fn connect_vpn(&self, request: ConnectVpnRequest) -> Result<(), ConnectivityError>;

    async fn disconnect_vpn(&self, profile_id: &str) -> Result<(), ConnectivityError>;

    async fn get_vpn_status(&self, profile_id: &str) -> Result<VpnStatus, ConnectivityError>;
}
