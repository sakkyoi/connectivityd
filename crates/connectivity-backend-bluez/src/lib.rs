use async_trait::async_trait;
use connectivity_backend::bluetooth_peripheral::BluetoothPeripheralBackend;
use connectivity_domain::{
    bluetooth::peripheral::{
        BluetoothAdapter, GattApplicationDef, NotifyRequest, StartAdvertisingRequest,
    },
    ConnectivityError,
};

pub struct BluezPeripheralBackend;

impl BluezPeripheralBackend {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl BluetoothPeripheralBackend for BluezPeripheralBackend {
    async fn list_adapters(&self) -> Result<Vec<BluetoothAdapter>, ConnectivityError> {
        Ok(vec![])
    }

    async fn get_adapter(&self, _adapter_id: &str) -> Result<BluetoothAdapter, ConnectivityError> {
        Err(ConnectivityError::Unsupported)
    }

    async fn set_powered(
        &self,
        _adapter_id: &str,
        _powered: bool,
    ) -> Result<(), ConnectivityError> {
        Ok(())
    }

    async fn start_advertising(
        &self,
        _request: StartAdvertisingRequest,
    ) -> Result<(), ConnectivityError> {
        Ok(())
    }

    async fn stop_advertising(
        &self,
        _advertisement_id: &str,
    ) -> Result<(), ConnectivityError> {
        Ok(())
    }

    async fn register_gatt_application(
        &self,
        _app: GattApplicationDef,
    ) -> Result<(), ConnectivityError> {
        Ok(())
    }

    async fn unregister_gatt_application(
        &self,
        _app_id: &str,
    ) -> Result<(), ConnectivityError> {
        Ok(())
    }

    async fn notify(&self, _request: NotifyRequest) -> Result<(), ConnectivityError> {
        Ok(())
    }
}
