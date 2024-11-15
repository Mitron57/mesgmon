use crate::domain::interfaces;
use crate::domain::interfaces::Database;
use axum::async_trait;
use std::sync::Arc;
use uuid::Uuid;
type Error = Box<dyn std::error::Error + Sync + Send>;
pub struct Repository<T> {
    pub storage: Arc<dyn Database<T, Error = Error> + Sync + Send>,
}

#[async_trait]
impl<T: Send + Sync> interfaces::Repository<T> for Repository<T> {
    type Error = Error;
    type Id = Uuid;

    async fn add(&self, item: T) -> Result<(), Self::Error> {
        self.storage.add(item).await
    }

    async fn remove(&self, id: Self::Id) -> Result<(), Self::Error> {
        self.storage.delete(id).await
    }

    async fn get(&self, id: Self::Id) -> Result<Option<T>, Self::Error> {
        self.storage.get(id).await
    }

    async fn update(&self, item: T) -> Result<(), Self::Error> {
        self.storage.update(item).await
    }
}
