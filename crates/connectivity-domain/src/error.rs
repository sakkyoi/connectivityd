use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ConnectivityError {
    #[error("permission denied")]
    PermissionDenied,

    #[error("resource busy")]
    Busy,

    #[error("adapter unavailable")]
    AdapterUnavailable,

    #[error("interface not found")]
    InterfaceNotFound,

    #[error("profile not found")]
    ProfileNotFound,

    #[error("network not found")]
    NetworkNotFound,

    #[error("invalid credential")]
    InvalidCredential,

    #[error("invalid configuration")]
    InvalidConfiguration,

    #[error("vpn not supported")]
    VpnNotSupported,

    #[error("bluetooth peripheral unavailable")]
    BluetoothPeripheralUnavailable,

    #[error("gatt service not found")]
    GattServiceNotFound,

    #[error("gatt characteristic not found")]
    GattCharacteristicNotFound,

    #[error("operation timeout")]
    OperationTimeout,

    #[error("unsupported operation")]
    Unsupported,

    #[error("backend failure: {0}")]
    BackendFailure(String),
}
