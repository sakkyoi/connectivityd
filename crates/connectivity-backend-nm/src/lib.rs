pub mod nm;

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
use zbus::{zvariant::OwnedObjectPath, Connection};

use nm::{
    access_point::NmAccessPoint,
    active_connection::is_root_path,
    connection_builder::{build_wifi_connection_settings, ov},
    device::NmDevice,
    manager::NmManager,
    mapping::{
        map_device_state_simple, map_device_type, map_wifi_security, decode_ssid,
        NM_DEVICE_TYPE_WIFI,
    },
    settings::{NmOptionsMap, NmRoot},
    wireless::NmWirelessDevice,
};

pub fn map_zbus_err(e: zbus::Error) -> ConnectivityError {
    ConnectivityError::BackendFailure(e.to_string())
}

pub struct NetworkManagerBackend;

impl NetworkManagerBackend {
    pub fn new() -> Self {
        Self
    }

    pub async fn connection(&self) -> Result<Connection, ConnectivityError> {
        Connection::system()
            .await
            .map_err(map_zbus_err)
    }
}

#[async_trait]
impl NetworkBackend for NetworkManagerBackend {
    async fn list_interfaces(&self) -> Result<Vec<NetworkInterface>, ConnectivityError> {
        let conn = self.connection().await?;
        let manager = NmManager::new(&conn)
            .await
            .map_err(map_zbus_err)?;

        let device_paths = manager
            .get_devices()
            .await
            .map_err(map_zbus_err)?;

        let mut result = Vec::new();

        for path in device_paths {
            let dev = NmDevice::new(&conn, path)
                .await
                .map_err(map_zbus_err)?;

            let id = dev
                .interface()
                .await
                .map_err(map_zbus_err)?;

            let kind = map_device_type(
                dev.device_type()
                    .await
                    .map_err(map_zbus_err)?,
            );

            let state = map_device_state_simple(
                dev.state()
                    .await
                    .map_err(map_zbus_err)?,
            );

            let enabled = dev
                .managed()
                .await
                .map_err(map_zbus_err)?;

            let mac_address = dev.hw_address().await.ok();

            result.push(NetworkInterface {
                id: id.clone(),
                name: id,
                kind,
                enabled,
                state,
                carrier: None,
                mac_address,
            });
        }

        Ok(result)
    }

    async fn get_interface(&self, interface_id: &str) -> Result<NetworkInterface, ConnectivityError> {
        let interfaces = self.list_interfaces().await?;
        interfaces
            .into_iter()
            .find(|i| i.id == interface_id)
            .ok_or(ConnectivityError::InterfaceNotFound)
    }

    async fn set_interface_enabled(
        &self,
        _interface_id: &str,
        _enabled: bool,
    ) -> Result<(), ConnectivityError> {
        Err(ConnectivityError::Unsupported)
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

    async fn scan_wifi(&self, request: WifiScanRequest) -> Result<(), ConnectivityError> {
        let conn = self.connection().await?;
        let manager = NmManager::new(&conn)
            .await
            .map_err(map_zbus_err)?;

        let device_paths = manager
            .get_devices()
            .await
            .map_err(map_zbus_err)?;

        let mut triggered = false;

        for path in device_paths {
            let dev = NmDevice::new(&conn, path.clone())
                .await
                .map_err(map_zbus_err)?;

            let interface = dev
                .interface()
                .await
                .map_err(map_zbus_err)?;

            let device_type = dev
                .device_type()
                .await
                .map_err(map_zbus_err)?;

            if device_type != NM_DEVICE_TYPE_WIFI {
                continue;
            }

            if let Some(target) = &request.interface_id {
                if &interface != target {
                    continue;
                }
            }

            triggered = true;

            let wifi = NmWirelessDevice::new(&conn, path)
                .await
                .map_err(map_zbus_err)?;

            wifi.request_scan()
                .await
                .map_err(map_zbus_err)?;
        }

        if !triggered {
            return Err(ConnectivityError::InterfaceNotFound);
        }

        Ok(())
    }

    async fn list_visible_wifi_networks(
        &self,
        interface_id: Option<&str>,
    ) -> Result<Vec<WifiNetwork>, ConnectivityError> {
        let conn = self.connection().await?;
        let manager = NmManager::new(&conn)
            .await
            .map_err(map_zbus_err)?;

        let device_paths = manager
            .get_devices()
            .await
            .map_err(map_zbus_err)?;

        let mut result = Vec::new();

        for path in device_paths {
            let dev = NmDevice::new(&conn, path.clone())
                .await
                .map_err(map_zbus_err)?;

            let iface = dev
                .interface()
                .await
                .map_err(map_zbus_err)?;

            let device_type = dev
                .device_type()
                .await
                .map_err(map_zbus_err)?;

            if device_type != NM_DEVICE_TYPE_WIFI {
                continue;
            }

            if let Some(target) = interface_id {
                if iface != target {
                    continue;
                }
            }

            let wifi = NmWirelessDevice::new(&conn, path)
                .await
                .map_err(map_zbus_err)?;

            let active_ap = wifi.active_access_point().await.ok();
            let ap_paths = match wifi.all_access_points().await {
                Ok(paths) => paths,
                Err(_) => wifi
                    .access_points()
                    .await
                    .map_err(map_zbus_err)?,
            };

            for ap_path in ap_paths {
                let ap = NmAccessPoint::new(&conn, ap_path.clone())
                    .await
                    .map_err(map_zbus_err)?;

                let ssid = decode_ssid(
                    ap.ssid()
                        .await
                        .map_err(map_zbus_err)?,
                );

                let strength = ap
                    .strength()
                    .await
                    .map_err(map_zbus_err)?;

                let flags = ap.flags().await.unwrap_or(0);
                let wpa_flags = ap.wpa_flags().await.unwrap_or(0);
                let rsn_flags = ap.rsn_flags().await.unwrap_or(0);

                let is_connected = active_ap
                    .as_ref()
                    .map(|p| p == &ap_path)
                    .unwrap_or(false);

                result.push(WifiNetwork {
                    id: ap_path.to_string(),
                    ssid,
                    signal_strength: strength,
                    security: map_wifi_security(flags, wpa_flags, rsn_flags),
                    connected: is_connected,
                });
            }
        }

        Ok(result)
    }

    async fn connect_wifi(&self, request: WifiConnectRequest) -> Result<(), ConnectivityError> {
        let conn = self.connection().await?;
        let manager = NmManager::new(&conn)
            .await
            .map_err(map_zbus_err)?;
        let root = NmRoot::new(&conn)
            .await
            .map_err(map_zbus_err)?;

        let device_paths = manager
            .get_devices()
            .await
            .map_err(map_zbus_err)?;

        let mut target_device_path: Option<OwnedObjectPath> = None;
        let mut target_interface_name: Option<String> = None;
        let mut target_ap_path: Option<OwnedObjectPath> = None;

        for path in device_paths {
            let dev = NmDevice::new(&conn, path.clone())
                .await
                .map_err(map_zbus_err)?;

            let iface = dev
                .interface()
                .await
                .map_err(map_zbus_err)?;

            let device_type = dev
                .device_type()
                .await
                .map_err(map_zbus_err)?;

            if device_type != NM_DEVICE_TYPE_WIFI {
                continue;
            }

            if let Some(target_if) = &request.interface_id {
                if &iface != target_if {
                    continue;
                }
            }

            let wifi = NmWirelessDevice::new(&conn, path.clone())
                .await
                .map_err(map_zbus_err)?;

            let ap_paths = match wifi.all_access_points().await {
                Ok(paths) => paths,
                Err(_) => wifi
                    .access_points()
                    .await
                    .map_err(map_zbus_err)?,
            };

            for ap_path in ap_paths {
                let ap = NmAccessPoint::new(&conn, ap_path.clone())
                    .await
                    .map_err(map_zbus_err)?;

                let ssid = decode_ssid(
                    ap.ssid()
                        .await
                        .map_err(map_zbus_err)?,
                );

                if ssid == request.ssid {
                    target_device_path = Some(path.clone());
                    target_interface_name = Some(iface.clone());
                    target_ap_path = Some(ap_path.clone());
                    break;
                }
            }

            if target_ap_path.is_some() {
                break;
            }
        }

        let device_path = target_device_path.ok_or(ConnectivityError::InterfaceNotFound)?;
        let ap_path = target_ap_path.ok_or(ConnectivityError::NetworkNotFound)?;

        let settings =
            build_wifi_connection_settings(&request, target_interface_name.as_deref());

        let mut options = NmOptionsMap::new();
        options.insert("persist".to_string(), ov("disk".to_string()));

        let _reply = root
            .add_and_activate_connection2(settings, device_path, ap_path, options)
            .await
            .map_err(map_zbus_err)?;

        Ok(())
    }

    async fn disconnect_wifi(
        &self,
        interface_id: Option<&str>,
    ) -> Result<(), ConnectivityError> {
        let conn = self.connection().await?;
        let manager = NmManager::new(&conn)
            .await
            .map_err(map_zbus_err)?;
        let root = NmRoot::new(&conn)
            .await
            .map_err(map_zbus_err)?;

        let device_paths = manager
            .get_devices()
            .await
            .map_err(map_zbus_err)?;

        for path in device_paths {
            let dev = NmDevice::new(&conn, path)
                .await
                .map_err(map_zbus_err)?;

            let iface = dev
                .interface()
                .await
                .map_err(map_zbus_err)?;

            let device_type = dev
                .device_type()
                .await
                .map_err(map_zbus_err)?;

            if device_type != NM_DEVICE_TYPE_WIFI {
                continue;
            }

            if let Some(target_if) = interface_id {
                if iface != target_if {
                    continue;
                }
            }

            let active_connection = dev
                .active_connection()
                .await
                .map_err(map_zbus_err)?;

            if !is_root_path(&active_connection) {
                root.deactivate_connection(active_connection)
                    .await
                    .map_err(map_zbus_err)?;
            }

            return Ok(());
        }

        return Err(ConnectivityError::InterfaceNotFound);
    }

    async fn list_saved_wifi_networks(
        &self,
        _interface_id: Option<&str>,
    ) -> Result<Vec<SavedWifiNetwork>, ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn forget_wifi_network(&self, _network_id: &str) -> Result<(), ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

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
