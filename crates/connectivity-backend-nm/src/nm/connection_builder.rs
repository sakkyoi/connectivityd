/// https://networkmanager.dev/docs/api/1.40/nm-settings-nmcli.html
use std::collections::HashMap;

use connectivity_domain::network::{
    ip::{IpAssignment, Ipv4Config, Ipv6Config},
    wifi::WifiConnectRequest,
};
use uuid::Uuid;
use zbus::zvariant::{OwnedValue, Value, ObjectPath};

use crate::nm::settings::NmSettingsMap;

pub fn ov<T>(value: T) -> OwnedValue
where
    T: Into<Value<'static>>,
{
    OwnedValue::try_from(value.into())
        .unwrap_or_else(|e| panic!("failed to convert to OwnedValue: {e}"))
}

fn build_connection_section(id: &str, interface_name: Option<&ObjectPath>) -> HashMap<String, OwnedValue> {
    let mut section = HashMap::new();
    section.insert("id".to_string(), ov(id.to_string()));
    section.insert("type".to_string(), ov("802-11-wireless".to_string()));
    section.insert("uuid".to_string(), ov(Uuid::new_v4().to_string()));

    if let Some(interface_name) = interface_name {
        section.insert("interface-name".to_string(), ov(interface_name.to_string()));
    }

    section
}

fn build_wifi_section(req: &WifiConnectRequest) -> HashMap<String, OwnedValue> {
    let mut section = HashMap::new();
    section.insert("ssid".to_string(), ov(req.ssid.as_bytes().to_vec()));
    section.insert("mode".to_string(), ov("infrastructure".to_string()));
    section
}

fn build_wifi_security_section(passphrase: &str) -> HashMap<String, OwnedValue> {
    let mut section = HashMap::new();
    section.insert("key-mgmt".to_string(), ov("wpa-psk".to_string()));
    section.insert("psk".to_string(), ov(passphrase.to_string()));
    section
}

fn build_ipv4_section(ipv4: Option<&Ipv4Config>) -> HashMap<String, OwnedValue> {
    let mut section = HashMap::new();

    match ipv4.map(|v| &v.assignment) {
        None | Some(IpAssignment::Dhcp) => {
            section.insert("method".to_string(), ov("auto".to_string()));
        }
        Some(IpAssignment::Static) => {
            section.insert("method".to_string(), ov("manual".to_string()));

            if let (Some(address), Some(prefix_len)) = (
                ipv4.and_then(|v| v.address.clone()),
                ipv4.and_then(|v| v.prefix_len),
            ) {
                let mut addr = HashMap::<String, OwnedValue>::new();
                addr.insert("address".to_string(), ov(address));
                addr.insert("prefix".to_string(), ov(prefix_len as u32));
                section.insert("address-data".to_string(), ov(vec![addr]));
            }

            if let Some(gateway) = ipv4.and_then(|v| v.gateway.clone()) {
                section.insert("gateway".to_string(), ov(gateway));
            }

            if let Some(dns) = ipv4.map(|v| v.dns_servers.clone()) {
                if !dns.is_empty() {
                    let dns_maps = dns
                        .into_iter()
                        .map(|s| {
                            let mut m = HashMap::<String, OwnedValue>::new();
                            m.insert("address".to_string(), ov(s));
                            m
                        })
                        .collect::<Vec<_>>();
                    section.insert("dns-data".to_string(), ov(dns_maps));
                }
            }
        }
        Some(IpAssignment::Auto) => {
            section.insert("method".to_string(), ov("auto".to_string()));
        }
    }

    section
}

fn build_ipv6_section(ipv6: Option<&Ipv6Config>) -> HashMap<String, OwnedValue> {
    let mut section = HashMap::new();

    match ipv6.map(|v| &v.assignment) {
        None | Some(IpAssignment::Auto) => {
            section.insert("method".to_string(), ov("auto".to_string()));
        }
        Some(IpAssignment::Static) => {
            section.insert("method".to_string(), ov("manual".to_string()));

            if let Some(v6) = ipv6 {
                if !v6.addresses.is_empty() {
                    let addresses = v6
                        .addresses
                        .iter()
                        .map(|a| {
                            let mut m = HashMap::<String, OwnedValue>::new();
                            m.insert("address".to_string(), ov(a.address.clone()));
                            m.insert("prefix".to_string(), ov(a.prefix_len as u32));
                            m
                        })
                        .collect::<Vec<_>>();
                    section.insert("address-data".to_string(), ov(addresses));
                }

                if let Some(gateway) = v6.gateway.clone() {
                    section.insert("gateway".to_string(), ov(gateway));
                }

                if !v6.dns_servers.is_empty() {
                    let dns_maps = v6
                        .dns_servers
                        .iter()
                        .cloned()
                        .map(|s| {
                            let mut m = HashMap::<String, OwnedValue>::new();
                            m.insert("address".to_string(), ov(s));
                            m
                        })
                        .collect::<Vec<_>>();
                    section.insert("dns-data".to_string(), ov(dns_maps));
                }
            }
        }
        Some(IpAssignment::Dhcp) => {
            // it's need to separate into DHCPv6, for conservatively, stay auto now
            section.insert("method".to_string(), ov("auto".to_string()));
        }
    }

    section
}

pub fn build_wifi_connection_settings(
    req: &WifiConnectRequest,
    interface_name: Option<&ObjectPath>,
) -> NmSettingsMap {
    let mut settings = NmSettingsMap::new();

    let connection_id = format!("wifi-{}", req.ssid);

    settings.insert(
        "connection".to_string(),
        build_connection_section(&connection_id, interface_name),
    );
    settings.insert("802-11-wireless".to_string(), build_wifi_section(req));

    if let Some(passphrase) = req.passphrase.as_deref() {
        if !passphrase.is_empty() {
            settings.insert(
                "802-11-wireless-security".to_string(),
                build_wifi_security_section(passphrase),
            );
        }
    }

    settings.insert("ipv4".to_string(), build_ipv4_section(req.ipv4.as_ref()));
    settings.insert("ipv6".to_string(), build_ipv6_section(req.ipv6.as_ref()));

    settings
}
