use std::sync::Arc;

use connectivity_backend::network::NetworkBackend;
use connectivity_domain::{
    events::{Event, EventSource},
    network::{
        ethernet::{ApplyEthernetConfigRequest, EthernetConfig},
        interface::NetworkInterface,
        vpn::{ConnectVpnRequest, VpnProfile, VpnStatus},
        wifi::{SavedWifiNetwork, WifiConnectRequest, WifiNetwork, WifiScanRequest},
    },
    ConnectivityError,
};
use connectivity_events::EventBus;

#[derive(Clone)]
pub struct NetworkService {
    backend: Arc<dyn NetworkBackend>,
    events: EventBus,
}

impl NetworkService {
    pub fn new(backend: Arc<dyn NetworkBackend>, events: EventBus) -> Self {
        Self { backend, events }
    }

    //
    // Interface inventory / state
    //

    pub async fn list_interfaces(&self) -> Result<Vec<NetworkInterface>, ConnectivityError> {
        self.backend.list_interfaces().await
    }

    pub async fn get_interface(
        &self,
        interface_id: &str,
    ) -> Result<NetworkInterface, ConnectivityError> {
        self.backend.get_interface(interface_id).await
    }

    pub async fn set_interface_enabled(
        &self,
        interface_id: &str,
        enabled: bool,
    ) -> Result<(), ConnectivityError> {
        self.backend
            .set_interface_enabled(interface_id, enabled)
            .await?;

        self.events.publish(Event::new(
            EventSource::Network,
            "InterfaceEnabledChanged",
            interface_id.to_string(),
            format!(r#"{{"enabled":{enabled}}}"#),
        ));

        Ok(())
    }

    //
    // Ethernet
    //

    pub async fn get_ethernet_config(
        &self,
        interface_id: &str,
    ) -> Result<EthernetConfig, ConnectivityError> {
        self.backend.get_ethernet_config(interface_id).await
    }

    pub async fn apply_ethernet_config(
        &self,
        request: ApplyEthernetConfigRequest,
    ) -> Result<(), ConnectivityError> {
        let interface_id = request.interface_id.clone();

        self.backend.apply_ethernet_config(request).await?;

        self.events.publish(Event::new(
            EventSource::Network,
            "EthernetConfigApplied",
            interface_id,
            "{}",
        ));

        Ok(())
    }

    //
    // Wi-Fi
    //

    pub async fn scan_wifi(&self, request: WifiScanRequest) -> Result<(), ConnectivityError> {
        let interface_id = request
            .interface_id
            .clone()
            .unwrap_or_else(|| "wifi".to_string());

        self.backend.scan_wifi(request).await?;

        self.events.publish(Event::new(
            EventSource::Network,
            "WifiScanRequested",
            interface_id,
            "{}",
        ));

        Ok(())
    }

    pub async fn list_visible_wifi_networks(
        &self,
        interface_id: Option<&str>,
    ) -> Result<Vec<WifiNetwork>, ConnectivityError> {
        self.backend.list_visible_wifi_networks(interface_id).await
    }

    pub async fn connect_wifi(
        &self,
        request: WifiConnectRequest,
    ) -> Result<(), ConnectivityError> {
        let ssid = request.ssid.clone();

        self.backend.connect_wifi(request).await?;

        self.events.publish(Event::new(
            EventSource::Network,
            "WifiConnectRequested",
            ssid,
            "{}",
        ));

        Ok(())
    }

    pub async fn disconnect_wifi(
        &self,
        interface_id: Option<&str>,
    ) -> Result<(), ConnectivityError> {
        let resource_id = interface_id.unwrap_or("wifi").to_string();

        self.backend.disconnect_wifi(interface_id).await?;

        self.events.publish(Event::new(
            EventSource::Network,
            "WifiDisconnectedRequested",
            resource_id,
            "{}",
        ));

        Ok(())
    }

    pub async fn list_saved_wifi_profiles(
        &self,
        interface_id: Option<&str>,
    ) -> Result<Vec<SavedWifiNetwork>, ConnectivityError> {
        self.backend.list_saved_wifi_networks(interface_id).await
    }

    pub async fn forget_wifi_network(
        &self,
        network_id: &str,
    ) -> Result<(), ConnectivityError> {
        self.backend.forget_wifi_network(network_id).await?;

        self.events.publish(Event::new(
            EventSource::Network,
            "WifiNetworkForgotten",
            network_id.to_string(),
            "{}",
        ));

        Ok(())
    }

    //
    // VPN
    //

    pub async fn list_vpn_profiles(&self) -> Result<Vec<VpnProfile>, ConnectivityError> {
        self.backend.list_vpn_profiles().await
    }

    pub async fn connect_vpn(
        &self,
        request: ConnectVpnRequest,
    ) -> Result<(), ConnectivityError> {
        let profile_id = request.profile_id.clone();

        self.backend.connect_vpn(request).await?;

        self.events.publish(Event::new(
            EventSource::Network,
            "VpnConnectRequested",
            profile_id,
            "{}",
        ));

        Ok(())
    }

    pub async fn disconnect_vpn(
        &self,
        profile_id: &str,
    ) -> Result<(), ConnectivityError> {
        self.backend.disconnect_vpn(profile_id).await?;

        self.events.publish(Event::new(
            EventSource::Network,
            "VpnDisconnectRequested",
            profile_id.to_string(),
            "{}",
        ));

        Ok(())
    }

    pub async fn get_vpn_status(
        &self,
        profile_id: &str,
    ) -> Result<VpnStatus, ConnectivityError> {
        self.backend.get_vpn_status(profile_id).await
    }
}
