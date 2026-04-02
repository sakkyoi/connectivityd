use zbus::{zvariant::OwnedObjectPath, Connection, Proxy};

pub struct NmDevice<'a> {
    proxy: Proxy<'a>,
    path: OwnedObjectPath,
}

impl<'a> NmDevice<'a> {
    pub async fn new(conn: &'a Connection, path: OwnedObjectPath) -> zbus::Result<Self> {
        let proxy = Proxy::new(
            conn,
            "org.freedesktop.NetworkManager",
            path.clone(),
            "org.freedesktop.NetworkManager.Device",
        )
            .await?;

        Ok(Self { proxy, path })
    }

    pub fn path(&self) -> &OwnedObjectPath {
        &self.path
    }

    pub async fn interface(&self) -> zbus::Result<String> {
        self.proxy.get_property("Interface").await
    }

    pub async fn device_type(&self) -> zbus::Result<u32> {
        self.proxy.get_property("DeviceType").await
    }

    pub async fn state(&self) -> zbus::Result<u32> {
        self.proxy.get_property("State").await
    }

    pub async fn managed(&self) -> zbus::Result<bool> {
        self.proxy.get_property("Managed").await
    }

    pub async fn hw_address(&self) -> zbus::Result<String> {
        self.proxy.get_property("HwAddress").await
    }

    pub async fn ip4_config(&self) -> zbus::Result<OwnedObjectPath> {
        self.proxy.get_property("Ip4Config").await
    }

    pub async fn ip6_config(&self) -> zbus::Result<OwnedObjectPath> {
        self.proxy.get_property("Ip6Config").await
    }

    pub async fn active_connection(&self) -> zbus::Result<OwnedObjectPath> {
        self.proxy.get_property("ActiveConnection").await
    }
}
