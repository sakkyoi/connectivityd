use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventSource {
    Network,
    BluetoothPeripheral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub source: EventSource,
    pub kind: String,
    pub resource_id: String,
    pub payload_json: String,
}

impl Event {
    pub fn new(
        source: EventSource,
        kind: impl Into<String>,
        resource_id: impl Into<String>,
        payload_json: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            source,
            kind: kind.into(),
            resource_id: resource_id.into(),
            payload_json: payload_json.into(),
        }
    }
}
