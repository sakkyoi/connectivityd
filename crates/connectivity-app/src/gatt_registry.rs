use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{agent_bridge::RemoteGattAgentClient, gatt_handler::GattHandler};

pub enum RegisteredGattHandler {
    Local(Arc<dyn GattHandler>),
    Remote(Arc<dyn RemoteGattAgentClient>),
}

pub struct GattRegistry {
    handlers: RwLock<HashMap<(String, String, String), RegisteredGattHandler>>,
}

impl GattRegistry {
    pub fn new() -> Self {
        Self {
            handlers: RwLock::new(HashMap::new()),
        }
    }

    pub fn register_local(
        &self,
        app_id: String,
        service_id: String,
        characteristic_id: String,
        handler: Arc<dyn GattHandler>,
    ) {
        self.handlers
            .write()
            .expect("gatt registry poisoned")
            .insert((app_id, service_id, characteristic_id), RegisteredGattHandler::Local(handler));
    }

    pub fn register_remote(
        &self,
        app_id: String,
        service_id: String,
        characteristic_id: String,
        handler: Arc<dyn RemoteGattAgentClient>,
    ) {
        self.handlers
            .write()
            .expect("gatt registry poisoned")
            .insert((app_id, service_id, characteristic_id), RegisteredGattHandler::Remote(handler));
    }

    pub fn get(
        &self,
        app_id: &str,
        service_id: &str,
        characteristic_id: &str,
    ) -> Option<RegisteredGattHandler> {
        self.handlers
            .read()
            .expect("gatt registry poisoned")
            .get(&(app_id.to_string(), service_id.to_string(), characteristic_id.to_string()))
            .map(|handler| match handler {
                RegisteredGattHandler::Local(h) => RegisteredGattHandler::Local(h.clone()),
                RegisteredGattHandler::Remote(h) => RegisteredGattHandler::Remote(h.clone()),
            })
    }
}
