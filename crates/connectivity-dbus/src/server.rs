use anyhow::Result;
use connectivity_app::context::AppContext;
use std::sync::Arc;
use zbus::connection::Builder as ConnectionBuilder;

use crate::{
    advertising::AdvertisingObject,
    bluetooth::BluetoothObject,
    ethernet::EthernetObject,
    gatt_host::GattHostObject,
    network::{NetworkObject, WifiObject},
    vpn::VpnObject,
};

pub struct DbusServer {
    ctx: Arc<AppContext>,
}

impl DbusServer {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        Self { ctx }
    }

    pub async fn run(self) -> Result<()> {
        let _connection = ConnectionBuilder::system()?
            .name("com.example.Connectivity")?
            .serve_at("/com/example/Connectivity/Network", NetworkObject::new(self.ctx.clone()))?
            .serve_at("/com/example/Connectivity/Network/WiFi", WifiObject::new(self.ctx.clone()))?
            .serve_at("/com/example/Connectivity/Network/Ethernet", EthernetObject::new(self.ctx.clone()))?
            .serve_at("/com/example/Connectivity/Network/VPN", VpnObject::new(self.ctx.clone()))?
            .serve_at("/com/example/Connectivity/Bluetooth", BluetoothObject::new(self.ctx.clone()))?
            .serve_at("/com/example/Connectivity/Bluetooth/Advertising", AdvertisingObject::new(self.ctx.clone()))?
            .serve_at("/com/example/Connectivity/Bluetooth/GattHost", GattHostObject::new(self.ctx.clone()))?
            .build()
            .await?;

        std::future::pending::<()>().await;
        #[allow(unreachable_code)]
        Ok(())
    }
}
