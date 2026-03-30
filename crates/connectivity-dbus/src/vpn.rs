use std::sync::Arc;

use connectivity_app::context::AppContext;
use connectivity_domain::network::{
    common::ConnectionState,
    vpn::{ConnectVpnRequest, VpnKind, VpnProfile, VpnStatus},
};
use serde::{Deserialize, Serialize};
use zbus::{fdo, interface, zvariant::Type};

use crate::error::map_domain_error;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct VpnProfileDto {
    pub id: String,
    pub kind: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct VpnStatusDto {
    pub profile_id: String,
    pub state: String,
}

impl From<VpnProfile> for VpnProfileDto {
    fn from(value: VpnProfile) -> Self {
        let kind = match value.kind {
            VpnKind::WireGuard => "wireguard",
            VpnKind::OpenVpn => "openvpn",
            VpnKind::Unknown => "unknown",
        }
            .to_string();

        Self {
            id: value.id,
            kind,
            display_name: value.display_name,
        }
    }
}

impl From<VpnStatus> for VpnStatusDto {
    fn from(value: VpnStatus) -> Self {
        let state = match value.state {
            ConnectionState::Disconnected => "disconnected",
            ConnectionState::Connecting => "connecting",
            ConnectionState::Connected => "connected",
            ConnectionState::Failed => "failed",
        }
            .to_string();

        Self {
            profile_id: value.profile_id,
            state,
        }
    }
}

pub struct VpnObject {
    ctx: Arc<AppContext>,
}

impl VpnObject {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        Self { ctx }
    }
}

#[interface(name = "com.example.Connectivity.Network.VPN1")]
impl VpnObject {
    async fn list_vpn_profiles(&self) -> fdo::Result<Vec<VpnProfileDto>> {
        let profiles = self
            .ctx
            .network
            .list_vpn_profiles()
            .await
            .map_err(map_domain_error)?;

        Ok(profiles.into_iter().map(Into::into).collect())
    }

    async fn connect_vpn(&self, profile_id: String) -> fdo::Result<()> {
        self.ctx
            .network
            .connect_vpn(ConnectVpnRequest { profile_id })
            .await
            .map_err(map_domain_error)
    }

    async fn disconnect_vpn(&self, profile_id: String) -> fdo::Result<()> {
        self.ctx
            .network
            .disconnect_vpn(&profile_id)
            .await
            .map_err(map_domain_error)
    }

    async fn get_vpn_status(&self, profile_id: String) -> fdo::Result<VpnStatusDto> {
        let status = self
            .ctx
            .network
            .get_vpn_status(&profile_id)
            .await
            .map_err(map_domain_error)?;

        Ok(status.into())
    }
}
