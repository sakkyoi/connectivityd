mod mapping;
mod security;

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
use mapping::{map_device, map_visible_wifi};
use nmrs::{NetworkManager, ConnectionError};
use security::map_connect_security;

use tokio::{task, sync::oneshot};

pub fn map_zbus_err(e: ConnectionError) -> ConnectivityError {
    ConnectivityError::BackendFailure(e.to_string())
}

pub struct NmrsNetworkBackend;

impl NmrsNetworkBackend {

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
impl NetworkBackend for NmrsNetworkBackend {
    //
    // Interface inventory
    //

    async fn list_interfaces(&self) -> Result<Vec<NetworkInterface>, ConnectivityError> {
        let nm = self.nm().await?;
        let devices = nm
            .list_devices()
            .await
            .map_err(map_zbus_err)?;

        Ok(devices
            .into_iter()
            .map(map_device)
            .collect())
    }

    async fn get_interface(&self, interface_id: &str) -> Result<NetworkInterface, ConnectivityError> {
        let interfaces = self.list_interfaces().await?;
        interfaces
            .into_iter()
            .find(|iface| iface.id == interface_id)
            .ok_or(ConnectivityError::InterfaceNotFound)
    }

    //
    // Wi-Fi control
    //

    async fn set_wifi_enabled(
        &self,
        _interface_id: Option<&str>, // TODO: maybe warning for nmrs didn't support specific interface for wifi related function
        enabled: bool,
    ) -> Result<(), ConnectivityError> {
        let nm = self.nm().await?;
        nm.set_wifi_enabled(enabled)
            .await
            .map_err(map_zbus_err)
    }

    async fn list_visible_wifi_networks(
        &self,
        _interface_id: Option<&str>,
    ) -> Result<Vec<WifiNetwork>, ConnectivityError> {
        let nm = self.nm().await?;
        let (tx, rx) = oneshot::channel();

        task::spawn_blocking(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|_| ConnectivityError::BackendFailure("internal error".into()))?;

            rt.block_on(async {
                let local = task::LocalSet::new();
                local.run_until(async move {
                    let result = nm.list_networks().await;
                    let _ = tx.send(result);
                }).await;
            });
            Ok::<(), ConnectivityError>(())
        });

        let nm = self.nm().await?;

        let current_ssid = nm
            .current_ssid()
            .await;

        let networks = rx.await
            .map_err(|_| ConnectivityError::BackendFailure("internal error".into()))?
            .map_err(map_zbus_err)?;

        Ok(networks
            .into_iter()
            .map(|network| map_visible_wifi(network, current_ssid.as_deref()))
            .collect())
    }

    async fn connect_wifi(&self, request: WifiConnectRequest) -> Result<(), ConnectivityError> {
        let nm = self.nm().await?;
        let security = map_connect_security(&request)?;

        nm.connect(&request.ssid, security)
            .await
            .map_err(map_zbus_err)
    }

    async fn disconnect_wifi(&self, _interface_id: Option<&str>) -> Result<(), ConnectivityError> {
        let nm = self.nm().await?;

        nm.disconnect()
            .await
            .map_err(map_zbus_err)
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

    async fn get_ipv4_config(
        &self,
        _interface_id: &str,
    ) -> Result<Option<Ipv4Config>, ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn set_ipv4_config(
        &self,
        _interface_id: &str,
        _config: Ipv4Config,
    ) -> Result<(), ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn get_ipv6_config(
        &self,
        _interface_id: &str,
    ) -> Result<Option<Ipv6Config>, ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn set_ipv6_config(
        &self,
        _interface_id: &str,
        _config: Ipv6Config,
    ) -> Result<(), ConnectivityError> {
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
