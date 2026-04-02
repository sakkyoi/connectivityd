use zbus::zvariant::OwnedObjectPath;

/// https://networkmanager.dev/docs/api/1.44.4/gdbus-org.freedesktop.NetworkManager.Device.html#gdbus-property-org-freedesktop-NetworkManager-Device.ActiveConnection
pub fn is_root_path(path: &OwnedObjectPath) -> bool {
    path.as_str() == "/"
}
