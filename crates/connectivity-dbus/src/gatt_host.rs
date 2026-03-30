use std::sync::Arc;

use connectivity_app::context::AppContext;
use connectivity_domain::bluetooth::peripheral::{
    GattApplicationDef, GattCharacteristicDef, GattServiceDef, NotifyRequest,
};
use serde::{Deserialize, Serialize};
use zbus::{fdo, interface, zvariant::Type};

use crate::error::map_domain_error;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct GattCharacteristicDefDto {
    pub characteristic_id: String,
    pub uuid: String,
    pub read: bool,
    pub write: bool,
    pub notify: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct GattServiceDefDto {
    pub service_id: String,
    pub uuid: String,
    pub primary: bool,
    pub characteristics: Vec<GattCharacteristicDefDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct GattApplicationDefDto {
    pub app_id: String,
    pub services: Vec<GattServiceDefDto>,
}

pub struct GattHostObject {
    ctx: Arc<AppContext>,
}

impl GattHostObject {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        Self { ctx }
    }
}

#[interface(name = "com.example.Connectivity.Bluetooth.GattHost1")]
impl GattHostObject {
    async fn register_gatt_application(&self, app: GattApplicationDefDto) -> fdo::Result<()> {
        let app = GattApplicationDef {
            app_id: app.app_id,
            services: app
                .services
                .into_iter()
                .map(|s| GattServiceDef {
                    service_id: s.service_id,
                    uuid: s.uuid,
                    primary: s.primary,
                    characteristic: s
                        .characteristics
                        .into_iter()
                        .map(|c| GattCharacteristicDef {
                            characteristic_id: c.characteristic_id,
                            uuid: c.uuid,
                            read: c.read,
                            write: c.write,
                            notify: c.notify,
                        })
                        .collect()
                })
                .collect(),
        };

        self.ctx
            .bluetooth_peripheral
            .register_gatt_application(app)
            .await
            .map_err(map_domain_error)
    }

    async fn unregister_gatt_application(&self, app_id: String) -> fdo::Result<()> {
        self.ctx
            .bluetooth_peripheral
            .unregister_gatt_application(&app_id)
            .await
            .map_err(map_domain_error)
    }

    async fn notify(
        &self,
        app_id: String,
        service_id: String,
        characteristic_id: String,
        value: Vec<u8>,
    ) -> fdo::Result<()> {
        self.ctx
            .bluetooth_peripheral
            .notify(NotifyRequest {
                app_id,
                service_id,
                characteristic_id,
                value,
            })
            .await
            .map_err(map_domain_error)
    }
}
