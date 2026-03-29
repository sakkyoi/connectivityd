use zbus::{zvariant::OwnedObjectPath, Connection, Proxy};

pub struct NmManager<'a> {
    proxy: Proxy<'a>,
}

impl<'a> NmManager<'a> {
    pub async fn new(conn: &'a Connection) -> zbus::Result<Self> {
        let proxy = Proxy::new(
            conn,
            "org.freedesktop.NetworkManager",
            "/org/freedesktop/NetworkManager",
            "org.freedesktop.NetworkManager",
        )
            .await?;

        Ok(Self { proxy })
    }

    pub async fn get_devices(&self) -> zbus::Result<Vec<OwnedObjectPath>> {
        self.proxy.get_property("Devices").await
    }
}
