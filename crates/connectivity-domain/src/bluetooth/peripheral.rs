use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BluetoothAdapter {
    pub id: String,
    pub name: String,
    pub powered: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvertisementData {
    pub local_name: Option<String>,
    pub service_uuids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartAdvertisingRequest {
    pub adapter_id: Option<String>,
    pub advertisement_id: String,
    pub data: AdvertisementData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GattApplicationDef {
    pub app_id: String,
    pub services: Vec<GattServiceDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GattServiceDef {
    pub service_id: String,
    pub uuid: String,
    pub primary: bool,
    pub characteristic: Vec<GattCharacteristicDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GattCharacteristicDef {
    pub characteristic_id: String,
    pub uuid: String,
    pub read: bool,
    pub write: bool,
    pub notify: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyRequest {
    pub app_id: String,
    pub service_id: String,
    pub characteristic_id: String,
    pub value: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacteristicReadRequest {
    pub connection_id: String,
    pub app_id: String,
    pub service_id: String,
    pub characteristic_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacteristicReadResponse {
    pub value: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacteristicWriteRequest {
    pub connection_id: String,
    pub app_id: String,
    pub service_id: String,
    pub characteristic_id: String,
    pub value: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacteristicWriteResponse {
    pub value: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionChangeRequest {
    pub connection_id: String,
    pub app_id: String,
    pub service_id: String,
    pub characteristic_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BluetoothPeripheralEvent {
    ClientConnected {
        connection_id: String,
        peer: String,
    },
    ClientDisconnected {
        connection_id: String,
    },
    ReadRequested {
        connection_id: String,
        app_id: String,
        service_id: String,
        characteristic_id: String,
    },
    WriteRequested {
        connection_id: String,
        app_id: String,
        service_id: String,
        characteristic_id: String,
    },
    SubscriptionChanged {
        connection_id: String,
        app_id: String,
        service_id: String,
        characteristic_id: String,
        subscribed: bool,
    },
}
