use axum::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Database<I> {
    type Error;
    async fn add(&self, item: I) -> Result<(), Self::Error>;
    async fn delete(&self, id: Uuid) -> Result<(), Self::Error>;
    async fn update(&self, item: I) -> Result<(), Self::Error>;
    
    async fn get(&self, id: Uuid) -> Result<Option<I>, Self::Error>;
}