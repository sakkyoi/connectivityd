use std::sync::Arc;

use connectivity_backend::network::NetworkBackend;
use connectivity_domain::{
    events::{Event, EventSource},
    network::{
        interface::NetworkInterface,
        ip::{Ipv4Config, Ipv6Config},
        vpn::{ConnectVpnRequest, VpnProfile, VpnStatus},
        wifi::{SavedWifiNetwork, WifiConnectRequest, WifiNetwork},
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
    // Interface inventory
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

    //
    // Wi-Fi control
    //

    pub async fn set_wifi_enabled(
        &self,
        interface_id: Option<&str>,
        wifi_enabled: bool,
    ) -> Result<(), ConnectivityError> {
        self.backend.set_wifi_enabled(interface_id, wifi_enabled).await
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
    // IP config
    //

    pub async fn get_ipv4_config(
        &self,
        interface_id: &str,
    ) -> Result<Option<Ipv4Config>, ConnectivityError> {
        self.backend.get_ipv4_config(interface_id).await
    }

    pub async fn set_ipv4_config(
        &self,
        interface_id: &str,
        ipv4_config: Ipv4Config,
    ) -> Result<(), ConnectivityError> {
        self.backend.set_ipv4_config(interface_id, ipv4_config).await?;

        self.events.publish(Event::new(
            EventSource::Network,
            "Ipv4ConfigUpdated",
            interface_id.to_string(),
            "{}",
        ));

        Ok(())
    }

    pub async fn get_ipv6_config(
        &self,
        interface_id: &str,
    ) -> Result<Option<Ipv6Config>, ConnectivityError> {
        self.backend.get_ipv6_config(interface_id).await
    }

    pub async fn set_ipv6_config(
        &self,
        interface_id: &str,
        config: Ipv6Config,
    ) -> Result<(), ConnectivityError> {
        self.backend.set_ipv6_config(interface_id, config).await?;

        self.events.publish(Event::new(
            EventSource::Network,
            "Ipv6ConfigUpdated",
            interface_id.to_string(),
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
