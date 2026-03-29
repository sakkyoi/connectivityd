use std::sync::Arc;

use connectivity_domain::{
    bluetooth::peripheral::{
        CharacteristicReadRequest, CharacteristicReadResponse, CharacteristicWriteRequest,
        CharacteristicWriteResponse, SubscriptionChangeRequest,
    },
    ConnectivityError,
};

use crate::gatt_registry::{GattRegistry, RegisteredGattHandler};

#[derive(Clone)]
pub struct GattDispatcher {
    registry: Arc<GattRegistry>,
}

impl GattDispatcher {
    pub fn new(registry: Arc<GattRegistry>) -> Self {
        Self { registry }
    }

    pub async fn dispatch_read(
        &self,
        request: CharacteristicReadRequest,
    ) -> Result<CharacteristicReadResponse, ConnectivityError> {
        match self
            .registry
            .get(&request.app_id, &request.service_id, &request.characteristic_id)
        {
            Some(RegisteredGattHandler::Local(handler)) => handler.on_read(request).await,
            Some(RegisteredGattHandler::Remote(agent)) => agent.read(request).await,
            None => Err(ConnectivityError::GattCharacteristicNotFound),
        }
    }

    pub async fn dispatch_write(
        &self,
        request: CharacteristicWriteRequest,
    ) -> Result<CharacteristicWriteResponse, ConnectivityError> {
        match self
            .registry
            .get(&request.app_id, &request.service_id, &request.characteristic_id)
        {
            Some(RegisteredGattHandler::Local(handler)) => handler.on_write(request).await,
            Some(RegisteredGattHandler::Remote(agent)) => agent.write(request).await,
            None => Err(ConnectivityError::GattCharacteristicNotFound),
        }
    }

    pub async fn dispatch_subscribe(
        &self,
        request: SubscriptionChangeRequest,
    ) -> Result<(), ConnectivityError> {
        match self
            .registry
            .get(&request.app_id, &request.service_id, &request.characteristic_id)
        {
            Some(RegisteredGattHandler::Local(handler)) => handler.on_subscribe(request).await,
            Some(RegisteredGattHandler::Remote(agent)) => agent.subscribe(request).await,
            None => Err(ConnectivityError::GattCharacteristicNotFound),
        }
    }

    pub async fn dispatch_unsubscribe(
        &self,
        request: SubscriptionChangeRequest,
    ) -> Result<(), ConnectivityError> {
        match self
            .registry
            .get(&request.app_id, &request.service_id, &request.characteristic_id)
        {
            Some(RegisteredGattHandler::Local(handler)) => handler.on_unsubscribe(request).await,
            Some(RegisteredGattHandler::Remote(agent)) => agent.unsubscribe(request).await,
            None => Err(ConnectivityError::GattCharacteristicNotFound),
        }
    }
}
