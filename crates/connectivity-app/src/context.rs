use std::sync::Arc;

use connectivity_backend::{
    bluetooth_peripheral::BluetoothPeripheralBackend,
    network::NetworkBackend,
};
use connectivity_events::EventBus;

use crate::{
    bluetooth_peripheral_service::BluetoothPeripheralService,
    gatt_dispatcher::GattDispatcher,
    gatt_registry::GattRegistry,
    network_service::NetworkService,
    session_manager::SessionManager,
};

pub struct AppContext {
    pub network: NetworkService,
    pub bluetooth_peripheral: BluetoothPeripheralService,
    pub gatt_registry: Arc<GattRegistry>,
    pub gatt_dispatcher: GattDispatcher,
    pub sessions: Arc<SessionManager>,
    pub events: EventBus,
}

impl AppContext {
    pub fn new(
        network_backend: Arc<dyn NetworkBackend>,
        bluetooth_backend: Arc<dyn BluetoothPeripheralBackend>,
        events: EventBus,
    ) -> Self {
        let sessions = Arc::new(SessionManager::new());
        let gatt_registry = Arc::new(GattRegistry::new());
        let gatt_dispatcher = GattDispatcher::new(gatt_registry.clone());
        let network = NetworkService::new(network_backend, events.clone());
        let bluetooth_peripheral = BluetoothPeripheralService::new(bluetooth_backend, events.clone());

        Self {
            network,
            bluetooth_peripheral,
            gatt_registry,
            gatt_dispatcher,
            sessions,
            events,
        }
    }
}
