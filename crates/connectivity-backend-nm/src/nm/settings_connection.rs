use std::collections::HashMap;

use zbus::{
    zvariant::{OwnedObjectPath, OwnedValue},
    Connection, Proxy,
};

pub type NmConnectionSettings = HashMap<String, HashMap<String, OwnedValue>>;

pub struct NmSettings<'a> {
    proxy: Proxy<'a>
}

impl<'a> NmSettings<'a> {
    pub async fn new(conn: &'a Connection) -> zbus::Result<Self> {
        let proxy = Proxy::new(
            conn,
            "org.freedesktop.NetworkManager",
            "/org/freedesktop/NetworkManager/Settings",
            "org.freedesktop.NetworkManager.Settings",
        )
            .await?;

        Ok(Self { proxy })
    }

    pub async fn list_connections(&self) -> zbus::Result<Vec<OwnedObjectPath>> {
        self.proxy.call("ListConnections", &()).await
    }
}

pub struct NmSettingsConnection<'a> {
    proxy: Proxy<'a>,
}

impl<'a> NmSettingsConnection<'a> {
    pub async fn new(conn: &'a Connection, path: OwnedObjectPath) -> zbus::Result<Self> {
        let proxy = Proxy::new(
            conn,
            "org.freedesktop.NetworkManager",
            path,
            "org.freedesktop.NetworkManager.Settings.Connection",
        )
            .await?;

        Ok(Self { proxy })
    }

    pub async fn get_settings(&self) -> zbus::Result<NmConnectionSettings> {
        self.proxy.call("GetSettings", &()).await
    }

    pub async fn update(&self, settings: NmConnectionSettings) -> zbus::Result<()> {
        let _: () = self.proxy.call("Update", &(settings)).await?;
        Ok(())
    }

    pub async fn delete(&self) -> zbus::Result<()> {
        let _: () = self.proxy.call("Delete", &()).await?;
        Ok(())
    }
}