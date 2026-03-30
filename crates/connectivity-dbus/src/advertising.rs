use std::sync::Arc;

use connectivity_app::context::AppContext;
use connectivity_domain::bluetooth::peripheral::{AdvertisementData, StartAdvertisingRequest};
use serde::{Deserialize, Serialize};
use zbus::{fdo, interface, zvariant::{Type, Optional}};

use crate::error::map_domain_error;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct AdvertisementDataDto {
    pub local_name: String,
    pub service_uuids: Vec<String>,
}

pub struct AdvertisingObject {
    ctx: Arc<AppContext>,
}

impl AdvertisingObject {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        Self { ctx }
    }
}

#[interface(name = "com.example.Connectivity.Bluetooth.Advertising1")]
impl AdvertisingObject {
    async fn start_advertising(
        &self,
        adapter_id: Optional<String>,
        advertisement_id: String,
        data: AdvertisementDataDto,
    ) -> fdo::Result<()> {
        let request = StartAdvertisingRequest {
            adapter_id: adapter_id.into(),
            advertisement_id,
            data: AdvertisementData {
                local_name: if data.local_name.is_empty() {
                    None
                } else {
                    Some(data.local_name)
                },
                service_uuids: data.service_uuids,
            },
        };

        self.ctx
            .bluetooth_peripheral
            .start_advertising(request)
            .await
            .map_err(map_domain_error)
    }

    async fn stop_advertising(&self, advertising_id: String) -> fdo::Result<()> {
        self.ctx
            .bluetooth_peripheral
            .stop_advertising(&advertising_id)
            .await
            .map_err(map_domain_error)
    }
}
