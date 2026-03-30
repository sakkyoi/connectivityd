use std::sync::Arc;

use connectivity_app::context::AppContext;
use connectivity_domain::bluetooth::peripheral::BluetoothAdapter;
use serde::{Deserialize, Serialize};
use zbus::{fdo, interface, zvariant::Type};

use crate::error::map_domain_error;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct BluetoothAdapterDto {
    pub id: String,
    pub name: String,
    pub powered: bool,
}

impl From<BluetoothAdapter> for BluetoothAdapterDto {
    fn from(value: BluetoothAdapter) -> Self {
        Self {
            id: value.id,
            name: value.name,
            powered: value.powered,
        }
    }
}

pub struct BluetoothObject {
    ctx: Arc<AppContext>,
}

impl BluetoothObject {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        Self { ctx }
    }
}

#[interface(name = "com.example.Connectivity.Bluetooth1")]
impl BluetoothObject {
    async fn list_adapters(&self) -> fdo::Result<Vec<BluetoothAdapterDto>> {
        let adapters = self
            .ctx
            .bluetooth_peripheral
            .list_adapters()
            .await
            .map_err(map_domain_error)?;

        Ok(adapters.into_iter().map(Into::into).collect())
    }

    async fn get_adapter(&self, adapter_id: String) -> fdo::Result<BluetoothAdapterDto> {
        let adapter = self
            .ctx
            .bluetooth_peripheral
            .get_adapter(&adapter_id)
            .await
            .map_err(map_domain_error)?;

        Ok(adapter.into())
    }

    async fn set_powered(&self, adapter_id: String, powered: bool) -> fdo::Result<()> {
        self.ctx
            .bluetooth_peripheral
            .set_powered(&adapter_id, powered)
            .await
            .map_err(map_domain_error)
    }
}
