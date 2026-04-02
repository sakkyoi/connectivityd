use std::collections::HashMap;

use zbus::{
    zvariant::{OwnedObjectPath, OwnedValue},
    Connection, Proxy,
};

pub type NmSettingsMap = HashMap<String, HashMap<String, OwnedValue>>;
pub type NmOptionsMap = HashMap<String, OwnedValue>;

pub struct NmRoot<'a> {
    proxy: Proxy<'a>,
}

impl<'a> NmRoot<'a> {
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

    /// https://networkmanager.dev/docs/api/latest/gdbus-org.freedesktop.NetworkManager.html#gdbus-method-org-freedesktop-NetworkManager.ActivateConnection
    pub async fn activate_connection(
        &self,
        connection: OwnedObjectPath,
        device: OwnedObjectPath,
        specific_object: OwnedObjectPath,
    ) -> zbus::Result<OwnedObjectPath> {
        self.proxy
            .call("ActivateConnection", &(connection, device, specific_object))
            .await
    }

    /// https://networkmanager.dev/docs/api/latest/gdbus-org.freedesktop.NetworkManager.html#gdbus-method-org-freedesktop-NetworkManager.AddAndActivateConnection2
    pub async fn add_and_activate_connection2(
        &self,
        connection: NmSettingsMap,
        device: OwnedObjectPath,
        specific_object: OwnedObjectPath,
        options: NmOptionsMap,
    ) -> zbus::Result<(OwnedObjectPath, OwnedObjectPath, HashMap<String, OwnedValue>)> {
        self.proxy
            .call(
                "AddAndActivateConnection2",
                &(connection, device, specific_object, options),
            )
            .await
    }

    /// https://networkmanager.dev/docs/api/latest/gdbus-org.freedesktop.NetworkManager.html#gdbus-method-org-freedesktop-NetworkManager.DeactivateConnection
    pub async fn deactivate_connection(
        &self,
        active_connection: OwnedObjectPath,
    ) -> zbus::Result<()> {
        let _: () = self
            .proxy
            .call("DeactivateConnection", &(active_connection))
            .await?;
        Ok(())
    }
}
