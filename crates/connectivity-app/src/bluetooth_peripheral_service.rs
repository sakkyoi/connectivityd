use std::sync::Arc;

use connectivity_backend::bluetooth_peripheral::BluetoothPeripheralBackend;
use connectivity_domain::{
    bluetooth::peripheral::{
        BluetoothAdapter, GattApplicationDef, NotifyRequest, StartAdvertisingRequest,
    },
    events::{Event, EventSource},
    ConnectivityError,
};
use connectivity_events::EventBus;

#[derive(Clone)]
pub struct BluetoothPeripheralService {
    backend: Arc<dyn BluetoothPeripheralBackend>,
    events: EventBus,
}

impl BluetoothPeripheralService {
    pub fn new(backend: Arc<dyn BluetoothPeripheralBackend>, events: EventBus) -> Self {
        Self { backend, events }
    }

    //
    // Adapter inventory / power
    //

    pub async fn list_adapters(&self) -> Result<Vec<BluetoothAdapter>, ConnectivityError> {
        self.backend.list_adapters().await
    }

    pub async fn get_adapter(
        &self,
        adapter_id: &str,
    ) -> Result<BluetoothAdapter, ConnectivityError> {
        self.backend.get_adapter(adapter_id).await
    }

    pub async fn set_powered(
        &self,
        adapter_id: &str,
        powered: bool,
    ) -> Result<(), ConnectivityError> {
        self.backend.set_powered(adapter_id, powered).await?;

        self.events.publish(Event::new(
            EventSource::BluetoothPeripheral,
            "BluetoothAdapterPowerChanged",
            adapter_id.to_string(),
            format!(r#"{{"powered":{powered}}}"#),
        ));

        Ok(())
    }

    //
    // Advertising
    //

    pub async fn start_advertising(
        &self,
        request: StartAdvertisingRequest,
    ) -> Result<(), ConnectivityError> {
        let advertisement_id = request.advertisement_id.clone();

        self.backend.start_advertising(request).await?;

        self.events.publish(Event::new(
            EventSource::BluetoothPeripheral,
            "AdvertisingStarted",
            advertisement_id,
            "{}",
        ));

        Ok(())
    }

    pub async fn stop_advertising(
        &self,
        advertisement_id: &str,
    ) -> Result<(), ConnectivityError> {
        self.backend.stop_advertising(advertisement_id).await?;

        self.events.publish(Event::new(
            EventSource::BluetoothPeripheral,
            "AdvertisingStopped",
            advertisement_id.to_string(),
            "{}",
        ));

        Ok(())
    }

    //
    // Local GATT application hosting
    //

    pub async fn register_gatt_application(
        &self,
        app: GattApplicationDef,
    ) -> Result<(), ConnectivityError> {
        let app_id = app.app_id.clone();

        self.backend.register_gatt_application(app).await?;

        self.events.publish(Event::new(
            EventSource::BluetoothPeripheral,
            "GattApplicationRegistered",
            app_id,
            "{}",
        ));

        Ok(())
    }

    pub async fn unregister_gatt_application(
        &self,
        app_id: &str,
    ) -> Result<(), ConnectivityError> {
        self.backend.unregister_gatt_application(app_id).await?;

        self.events.publish(Event::new(
            EventSource::BluetoothPeripheral,
            "GattApplicationUnregistered",
            app_id.to_string(),
            "{}",
        ));

        Ok(())
    }

    //
    // Notifications
    //

    pub async fn notify(&self, request: NotifyRequest) -> Result<(), ConnectivityError> {
        let characteristic_id = request.characteristic_id.clone();

        self.backend.notify(request).await?;

        self.events.publish(Event::new(
            EventSource::BluetoothPeripheral,
            "GattNotificationSent",
            characteristic_id,
            "{}",
        ));

        Ok(())
    }
}
