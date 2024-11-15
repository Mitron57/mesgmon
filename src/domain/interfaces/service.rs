use axum::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Service<I, D> {
    type Error;
    type Repository;
    type MessageBroker;
    async fn create(
        &self,
        dto: D,
        repo: Self::Repository,
        broker: Self::MessageBroker,
    ) -> Result<I, Self::Error>;
    async fn update(
        &self,
        id: Uuid,
        dto: D,
        repo: Self::Repository,
        broker: Self::MessageBroker,
    ) -> Result<(), Self::Error>;
    async fn delete(
        &self,
        id: Uuid,
        repo: Self::Repository,
        broker: Self::MessageBroker,
    ) -> Result<(), Self::Error>;
}
