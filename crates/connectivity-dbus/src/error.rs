use connectivity_domain::ConnectivityError;
use zbus::fdo;

pub fn map_domain_error(err: ConnectivityError) -> fdo::Error {
    match err {
        ConnectivityError::PermissionDenied => {
            fdo::Error::AccessDenied("permission denied".into())
        }
        ConnectivityError::InterfaceNotFound => {
            fdo::Error::Failed("interface not found".into())
        }
        ConnectivityError::ProfileNotFound => {
            fdo::Error::Failed("profile not found".into())
        }
        ConnectivityError::NetworkNotFound => {
            fdo::Error::Failed("network not found".into())
        }
        ConnectivityError::GattServiceNotFound => {
            fdo::Error::Failed("gatt service not found".into())
        }
        ConnectivityError::GattCharacteristicNotFound => {
            fdo::Error::Failed("gatt characteristic not found".into())
        }
        ConnectivityError::Busy => {
            fdo::Error::Failed("busy".into())
        }
        ConnectivityError::InvalidConfiguration => {
            fdo::Error::InvalidArgs("invalid configuration".into())
        }
        ConnectivityError::InvalidCredential => {
            fdo::Error::InvalidArgs("invalid credential".into())
        }
        ConnectivityError::Unsupported => {
            fdo::Error::NotSupported("unsupported operation".into())
        }
        ConnectivityError::OperationTimeout => {
            fdo::Error::Timeout("operation timeout".into())
        }
        other => fdo::Error::Failed(other.to_string()),
    }
}
