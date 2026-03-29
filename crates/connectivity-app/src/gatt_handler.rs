use async_trait::async_trait;
use connectivity_domain::{
    bluetooth::peripheral::{
        CharacteristicReadRequest, CharacteristicReadResponse, CharacteristicWriteRequest,
        CharacteristicWriteResponse, SubscriptionChangeRequest,
    },
    ConnectivityError,
};

#[async_trait]
pub trait GattHandler: Send + Sync {
    async fn on_read(
        &self,
        request: CharacteristicReadRequest,
    ) -> Result<CharacteristicReadResponse, ConnectivityError>;

    async fn on_write(
        &self,
        request: CharacteristicWriteRequest,
    ) -> Result<CharacteristicWriteResponse, ConnectivityError>;

    async fn on_subscribe(
        &self,
        request: SubscriptionChangeRequest,
    ) -> Result<(), ConnectivityError>;

    async fn on_unsubscribe(
        &self,
        request: SubscriptionChangeRequest,
    ) -> Result<(), ConnectivityError>;
}
