use std::collections::HashMap;
use zbus::{zvariant::{OwnedObjectPath, OwnedValue}, Connection, Proxy};

/// https://networkmanager.dev/docs/api/latest/gdbus-org.freedesktop.NetworkManager.html
pub struct NmNetworkManager<'a> {
    proxy: Proxy<'a>,
}

impl<'a> NmNetworkManager<'a> {
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

    /// Reloads NetworkManager configuration and state.
    ///
    /// This method reloads configuration and performs updates such as flushing
    /// caches or rewriting external state. It is similar to sending `SIGHUP` to
    /// NetworkManager, but allows fine-grained control via `flags`, and is
    /// synchronous.
    ///
    /// # Parameters
    /// - `flags`: Bitflags controlling what to reload.
    ///
    ///   Common values:
    ///   - `0x00`: Reload everything (equivalent to `SIGHUP`)
    ///   - `0x01`: Reload `NetworkManager.conf` from disk
    ///   - `0x02`: Update DNS configuration (e.g. rewrite `/etc/resolv.conf`)
    ///   - `0x04`: Restart DNS plugin (may briefly interrupt name resolution)
    ///
    ///   Flags may have implicit effects. For example:
    ///   - Restarting the DNS plugin (`0x04`) also updates DNS (`0x02`)
    ///   - Reloading configuration (`0x01`) may trigger DNS updates if DNS-related
    ///     settings changed
    ///
    /// # Returns
    ///
    /// `Ok(())` if the reload operation completed successfully.
    pub async fn reload(&self, flags: u32) -> zbus::Result<()> {
        self.proxy.call("Reload", &(flags)).await
    }

    /// Returns the object paths of realized network devices.
    ///
    /// This method does not include device placeholders. Use
    /// [`Self::get_all_devices`] to retrieve all known devices, including
    /// placeholders.
    ///
    /// # Returns
    ///
    /// A vector of realized device object paths.
    pub async fn get_devices(&self) -> zbus::Result<Vec<OwnedObjectPath>> {
        self.proxy.call("GetDevices", &()).await
    }

    /// Returns the object paths of all network devices.
    ///
    /// This method includes both realized network devices and device placeholders.
    /// Device placeholders represent devices that do not yet exist, but may be
    /// automatically created by NetworkManager if one of their available
    /// connections is activated.
    ///
    /// Use [`Self::get_devices`] to retrieve only realized network devices.
    ///
    /// # Returns
    ///
    /// A vector of object paths for all network devices, including placeholders.
    pub async fn get_all_devices(&self) -> zbus::Result<Vec<OwnedObjectPath>> {
        self.proxy.call("GetAllDevices", &()).await
    }

    /// Returns the object path of a network device by its IP interface name.
    ///
    /// This method looks up a device using its IP interface name (e.g. `"eth0"`).
    /// Note that some devices (such as modems) only have an IP interface name.
    /// when ehry are connected.
    ///
    /// # Parameters
    ///
    /// - `iface`: The interface name of the device to find.
    ///
    /// # Returns
    ///
    /// The object path of the matching network device.
    pub async fn get_device_by_ip_iface(&self, iface: &str) -> zbus::Result<OwnedObjectPath> {
        self.proxy.call("GetDeviceByIpIface", &(iface)).await
    }

    pub async fn activate_connection(
        &self,
        connection: OwnedObjectPath,
        device: OwnedObjectPath,
        specific_object: OwnedObjectPath,
    ) -> zbus::Result<OwnedObjectPath> {
        todo!()
    }

    pub async fn add_and_activate_connection(
        &self,
        connection: HashMap<String, HashMap<String, OwnedValue>>,
        device: OwnedObjectPath,
        specific_object: OwnedObjectPath,
    ) -> zbus::Result<(OwnedObjectPath, OwnedObjectPath)> {
        todo!()
    }

    pub async fn add_and_activate_connection2(
        &self,
        connection: HashMap<String, HashMap<String, OwnedValue>>,
        device: OwnedObjectPath,
        specific_object: OwnedObjectPath,
        options: HashMap<String, OwnedValue>,
    ) -> zbus::Result<(OwnedObjectPath, OwnedObjectPath, HashMap<String, OwnedValue>)> {
        todo!()
    }

    pub async fn deactivate_connection(&self, active_connection: OwnedObjectPath) -> zbus::Result<()> {
        todo!()
    }

    pub async fn sleep(&self, sleep: bool) -> zbus::Result<()> {
        todo!()
    }

    pub async fn enable(&self, enable: bool) -> zbus::Result<()> {
        todo!()
    }

    pub async fn get_permissions(&self) -> zbus::Result<HashMap<String, String>> {
        todo!()
    }

    pub async fn set_logging(&self, level: &str, domains: &str) -> zbus::Result<()> {
        todo!()
    }

    pub async fn check_connectivity(&self) -> zbus::Result<u32> {
        todo!()
    }

    pub async fn state(&self) -> zbus::Result<u32> {
        todo!()
    }

    pub async fn checkpoint_create(
        &self,
        devices: Vec<OwnedObjectPath>,
        rollback_timeout: u32,
        flags: u32,
    ) -> zbus::Result<OwnedObjectPath> {
        todo!()
    }

    pub async fn checkpoint_destroy(&self, checkpoint: OwnedObjectPath) -> zbus::Result<()> {
        todo!()
    }

    pub async fn checkpoint_rollback(&self, checkpoint: OwnedObjectPath) -> zbus::Result<HashMap<String, u32>> {
        todo!()
    }

    pub async fn checkpoint_adjust_rollback_timeout(&self, checkpoint: OwnedObjectPath) -> zbus::Result<u32> {
        todo!()
    }
}
