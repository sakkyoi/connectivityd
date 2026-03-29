use std::sync::Arc;

use connectivity_backend::network::NetworkBackend;
use connectivity_domain::{
    events::{Event, EventSource},
    network::{
        ethernet::{ApplyEthernetConfigRequest, EthernetConfig},
        interface::NetworkInterface,
        vpn::{ConnectVpnRequest, VpnProfile},
        wifi::{SavedWifiProfile, WifiConnectRequest, WifiScanRequest, WifiVisibleNetwork},
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

    pub async fn list_interfaces(&self) -> Result<Vec<NetworkInterface>, ConnectivityError> {
        self.backend.list_interfaces().await
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

    pub async fn get_ethernet_config(
        &self,
        interface_id: &str,
    ) -> Result<EthernetConfig, ConnectivityError> {
        self.backend.get_ethernet_config(interface_id).await
    }

    pub async fn scan_wifi(&self, request: WifiScanRequest) -> Result<(), ConnectivityError> {
        self.backend.scan_wifi(request).await?;
        self.events.publish(Event::new(
            EventSource::Network,
            "WifiScanRequested",
            "wifi",
            "{}",
        ));
        Ok(())
    }

    pub async fn list_visible_wifi_networks(
        &self,
        interface_id: Option<&str>,
    ) -> Result<Vec<WifiVisibleNetwork>, ConnectivityError> {
        self.backend.list_visible_wifi_networks(interface_id).await
    }

    pub async fn connect_wifi(&self, request: WifiConnectRequest) -> Result<(), ConnectivityError> {
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

    pub async fn list_saved_wifi_profiles(
        &self,
        interface_id: Option<&str>,
    ) -> Result<Vec<SavedWifiProfile>, ConnectivityError> {
        self.backend.list_saved_wifi_profiles(interface_id).await
    }

    pub async fn list_vpn_profiles(&self) -> Result<Vec<VpnProfile>, ConnectivityError> {
        self.backend.list_vpn_profiles().await
    }

    pub async fn connect_vpn(&self, request: ConnectVpnRequest) -> Result<(), ConnectivityError> {
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
}
