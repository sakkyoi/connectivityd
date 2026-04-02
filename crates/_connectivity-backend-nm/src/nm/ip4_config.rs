use std::collections::HashMap;

use zbus::{zvariant::{OwnedObjectPath, OwnedValue}, Connection, Proxy};

pub struct NmIp4Config<'a> {
    proxy: Proxy<'a>,
}

impl<'a> NmIp4Config<'a> {
    pub async fn new(conn: &'a Connection, path: OwnedObjectPath) -> zbus::Result<Self> {
        let proxy = Proxy::new(
            conn,
            "org.freedesktop.NetworkManager",
            path,
            "org.freedesktop.NetworkManager.IP4Config",
        )
            .await?;

        Ok(Self { proxy })
    }

    pub async fn address_data(&self) -> zbus::Result<Vec<HashMap<String, OwnedValue>>> {
        self.proxy.get_property("AddressData").await
    }

    pub async fn gateway(&self) -> zbus::Result<String> {
        self.proxy.get_property("Gateway").await
    }

    pub async fn nameserver_data(&self) -> zbus::Result<Vec<HashMap<String, OwnedValue>>> {
        self.proxy.get_property("NameserverData").await
    }
}
