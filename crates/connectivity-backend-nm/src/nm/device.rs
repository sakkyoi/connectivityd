use std::collections::HashMap;
use futures_util::Stream;
use zbus::{
    zvariant::{OwnedObjectPath, OwnedValue},
    Connection, Proxy,
};

use crate::nm::{
    proxy_access::HasProxy,
    property::{Property, PropertyAccess, WritableProperty},
};
use crate::nm::signal::SignalAccess;

/// https://networkmanager.dev/docs/api/latest/gdbus-org.freedesktop.NetworkManager.Device.html
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

    pub fn props(&self) -> NmDeviceProps<'_, 'a> {
        NmDeviceProps { inner: self }
    }
}

impl<'a> NmDevice<'a> {
    // methods

    ///
    pub async fn reapply(
        &self,
        connection: HashMap<String, HashMap<String, OwnedValue>>,
        version_id: u64,
        flags: u32,
    ) -> zbus::Result<()> {
        self.proxy.call("Reapply", &(connection, version_id, flags)).await
    }

    ///
    pub async fn get_applied_connection(&self) -> zbus::Result<(HashMap<String, HashMap<String, OwnedValue>>, u64)> {
        self.proxy.call("GetAppliedConnection", &()).await
    }

    ///
    pub async fn disconnect(&self) -> zbus::Result<()> {
        self.proxy.call("Disconnect", &()).await
    }

    ///
    pub async fn delete(&self) -> zbus::Result<()> {
        self.proxy.call("Delete", &()).await
    }
}

pub struct NmDeviceProps<'d, 'a> {
    inner: &'d NmDevice<'a>,
}

impl<'d, 'a> HasProxy for NmDeviceProps<'d, 'a> {
    fn proxy(&self) -> &Proxy<'_> {
        &self.inner.proxy
    }
}

impl<'d, 'a> NmDeviceProps<'d, 'a> {
    pub fn udi(&self) -> Property<'_, Self, String> {
        self.prop("Udi")
    }

    pub fn path(&self) -> Property<'_, Self, String> {
        self.prop("Path")
    }

    pub fn interface(&self) -> Property<'_, Self, String> {
        self.prop("Interface")
    }

    pub fn ip_interface(&self) -> Property<'_, Self, String> {
        self.prop("IpInterface")
    }

    pub fn driver(&self) -> Property<'_, Self, String> {
        self.prop("Driver")
    }

    pub fn driver_version(&self) -> Property<'_, Self, String> {
        self.prop("DriverVersion")
    }

    pub fn firmware_version(&self) -> Property<'_, Self, String> {
        self.prop("FirmwareVersion")
    }

    pub fn capabilities(&self) -> Property<'_, Self, u32> {
        self.prop("Capabilities")
    }

    pub fn ip4_address(&self) -> Property<'_, Self, u32> {
        self.prop("Ip4Address")
    }

    pub fn state(&self) -> Property<'_, Self, u32> {
        self.prop("State")
    }

    pub fn state_reason(&self) -> Property<'_, Self, (u32, u32)> {
        self.prop("StateReason")
    }

    pub fn active_connection(&self) -> Property<'_, Self, OwnedObjectPath> {
        self.prop("ActiveConnection")
    }

    pub fn ip4_config(&self) -> Property<'_, Self, OwnedObjectPath> {
        self.prop("Ip4Config")
    }

    pub fn dhcp4_config(&self) -> Property<'_, Self, OwnedObjectPath> {
        self.prop("Dhcp4Config")
    }

    pub fn ip6_config(&self) -> Property<'_, Self, OwnedObjectPath> {
        self.prop("Ip6Config")
    }

    pub fn dhcp6_config(&self) -> Property<'_, Self, OwnedObjectPath> {
        self.prop("Dhcp6Config")
    }

    pub fn managed(&self) -> WritableProperty<'_, Self, bool> {
        self.prop_rw("Managed")
    }

    pub fn autoconnect(&self) -> WritableProperty<'_, Self, bool> {
        self.prop_rw("Autoconnect")
    }

    pub fn firmware_missing(&self) -> Property<'_, Self, bool> {
        self.prop("FirmwareMissing")
    }

    pub fn nm_plugin_missing(&self) -> Property<'_, Self, bool> {
        self.prop("NmPluginMissing")
    }

    pub fn device_type(&self) -> Property<'_, Self, u32> {
        self.prop("DeviceType")
    }

    pub fn available_connections(&self) -> Property<'_, Self, Vec<OwnedObjectPath>> {
        self.prop("AvailableConnections")
    }

    pub fn physical_port_id(&self) -> Property<'_, Self, String> {
        self.prop("PhysicalPortId")
    }

    pub fn mtu(&self) -> Property<'_, Self, u32> {
        self.prop("Mtu")
    }

    pub fn metered(&self) -> Property<'_, Self, u32> {
        self.prop("Metered")
    }

    pub fn lldp_neighbors(&self) -> Property<'_, Self, Vec<HashMap<String, OwnedValue>>> {
        self.prop("LldpNeighbors")
    }

    pub fn real(&self) -> Property<'_, Self, bool> {
        self.prop("Real")
    }

    pub fn ip4_connectivity(&self) -> Property<'_, Self, u32> {
        self.prop("Ip4Connectivity")
    }

    pub fn ip6_connectivity(&self) -> Property<'_, Self, u32> {
        self.prop("Ip6Connectivity")
    }

    pub fn interface_flags(&self) -> Property<'_, Self, u32> {
        self.prop("InterfaceFlags")
    }

    pub fn hw_address(&self) -> Property<'_, Self, String> {
        self.prop("HwAddress")
    }

    pub fn ports(&self) -> Property<'_, Self, Vec<OwnedObjectPath>> {
        self.prop("Ports")
    }
}

pub struct NmDeviceSignals<'d, 'a> {
    pub(crate) inner: &'d NmDevice<'a>,
}

impl<'d, 'a> HasProxy for NmDeviceSignals<'d, 'a> {
    fn proxy(&self) -> &Proxy<'_> {
        &self.inner.proxy
    }
}

impl<'d, 'a> NmDeviceSignals<'d, 'a> {
    pub async fn state_changed(
        &self,
    ) -> zbus::Result<impl Stream<Item = zbus::Result<(u32, u32, u32)>> + '_> {
        self.receive_signal_typed("StateChanged").await
    }
}
