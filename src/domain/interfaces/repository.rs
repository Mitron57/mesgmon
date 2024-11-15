use axum::async_trait;

#[async_trait]
pub trait Repository<I: Send + Sync> {
    type Error;
    type Id;
    async fn add(&self, item: I) -> Result<(), Self::Error>;
    async fn remove(&self, id: Self::Id) -> Result<(), Self::Error>;
    async fn get(&self, id: Self::Id) -> Result<Option<I>, Self::Error>;
    async fn update(&self, item: I) -> Result<(), Self::Error>;
}