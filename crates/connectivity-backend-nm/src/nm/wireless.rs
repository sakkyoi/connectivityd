use std::collections::HashMap;

use zbus::{zvariant::OwnedObjectPath, Connection, Proxy};

pub struct NmWirelessDevice<'a> {
    proxy: Proxy<'a>,
}

impl<'a> NmWirelessDevice<'a> {
    pub async fn new(conn: &'a Connection, path: OwnedObjectPath) -> zbus::Result<Self> {
        let proxy = Proxy::new(
            conn,
            "org.freedesktop.NetworkManager",
            path,
            "org.freedesktop.NetworkManager.Device.Wireless",
        )
            .await?;

        Ok(Self { proxy })
    }

    pub async fn request_scan(&self) -> zbus::Result<()> {
        let options: HashMap<&str, zbus::zvariant::Value<'_>> = HashMap::new();
        let _reply = self.proxy.call_method("RequestScan", &options).await?;
        Ok(())
    }

    pub async fn access_points(&self) -> zbus::Result<Vec<OwnedObjectPath>> {
        self.proxy.get_property("AccessPoints").await
    }

    pub async fn all_access_points(&self) -> zbus::Result<Vec<OwnedObjectPath>> {
        self.proxy.call("GetAllAccessPoints", &()).await
    }

    pub async fn active_access_point(&self) -> zbus::Result<OwnedObjectPath> {
        self.proxy.get_property("ActiveAccessPoint").await
    }

    pub async fn last_scan(&self) -> zbus::Result<i64> {
        self.proxy.get_property("LastScan").await
    }
}
