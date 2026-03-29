use async_trait::async_trait;
use connectivity_domain::{
    bluetooth::peripheral::{
        CharacteristicReadRequest, CharacteristicReadResponse, CharacteristicWriteRequest,
        CharacteristicWriteResponse, SubscriptionChangeRequest,
    },
    ConnectivityError,
};

#[async_trait]
pub trait RemoteGattAgentClient: Send + Sync {
    async fn read(
        &self,
        request: CharacteristicReadRequest,
    ) -> Result<CharacteristicReadResponse, ConnectivityError>;

    async fn write(
        &self,
        request: CharacteristicWriteRequest,
    ) -> Result<CharacteristicWriteResponse, ConnectivityError>;

    async fn subscribe(
        &self,
        request: SubscriptionChangeRequest,
    ) -> Result<(), ConnectivityError>;

    async fn unsubscribe(
        &self,
        request: SubscriptionChangeRequest,
    ) -> Result<(), ConnectivityError>;
}
