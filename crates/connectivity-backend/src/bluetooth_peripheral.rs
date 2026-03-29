use async_trait::async_trait;
use connectivity_domain::{
    bluetooth::peripheral::{
        BluetoothAdapter, GattApplicationDef, NotifyRequest, StartAdvertisingRequest,
    },
    ConnectivityError,
};

#[async_trait]
pub trait BluetoothPeripheralBackend: Send + Sync {
    async fn list_adapters(&self) -> Result<Vec<BluetoothAdapter>, ConnectivityError>;

    async fn set_powered(
        &self,
        adapter_id: &str,
        powered: bool,
    ) -> Result<(), ConnectivityError>;

    async fn start_advertising(
        &self,
        request: StartAdvertisingRequest,
    ) -> Result<(), ConnectivityError>;

    async fn stop_advertising(
        &self,
        advertisement_id: &str,
    ) -> Result<(), ConnectivityError>;

    async fn register_gatt_application(
        &self,
        app: GattApplicationDef,
    ) -> Result<(), ConnectivityError>;

    async fn unregister_gatt_application(
        &self,
        app_id: &str,
    ) -> Result<(), ConnectivityError>;

    async fn notify(&self, request: NotifyRequest) -> Result<(), ConnectivityError>;
}
