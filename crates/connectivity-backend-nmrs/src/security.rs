use connectivity_domain::{network::wifi::WifiConnectRequest, ConnectivityError};
use nmrs::WifiSecurity;

pub fn map_connect_security(
    req: &WifiConnectRequest,
) -> Result<WifiSecurity, ConnectivityError> {
    match req.passphrase.as_deref() {
        Some(psk) if !psk.is_empty() => Ok(WifiSecurity::WpaPsk {
            psk: psk.to_string(),
        }),
        _ => Ok(WifiSecurity::Open),
    }
}
