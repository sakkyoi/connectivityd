use zbus::{zvariant::OwnedObjectPath, Connection, Proxy};

pub struct NmAccessPoint<'a> {
    proxy: Proxy<'a>,
}

impl<'a> NmAccessPoint<'a> {
    pub async fn new(conn: &'a Connection, path: OwnedObjectPath) -> zbus::Result<Self> {
        let proxy = Proxy::new(
            conn,
            "org.freedesktop.NetworkManager",
            path,
            "org.freedesktop.NetworkManager.AccessPoint",
        )
            .await?;

        Ok(Self { proxy })
    }

    pub async fn ssid(&self) -> zbus::Result<Vec<u8>> {
        self.proxy.get_property("Ssid").await
    }

    pub async fn strength(&self) -> zbus::Result<u8> {
        self.proxy.get_property("Strength").await
    }

    pub async fn flags(&self) -> zbus::Result<u32> {
        self.proxy.get_property("Flags").await
    }

    pub async fn wpa_flags(&self) -> zbus::Result<u32> {
        self.proxy.get_property("WpaFlags").await
    }

    pub async fn rsn_flags(&self) -> zbus::Result<u32> {
        self.proxy.get_property("RsnFlags").await
    }
}
