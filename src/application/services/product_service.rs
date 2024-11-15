use crate::domain::dto::Description;
use crate::domain::interfaces;
use crate::domain::interfaces::{Identifiable, MessageBroker, Repository};
use crate::domain::models::Product;
use axum::async_trait;
use std::error::Error;
use std::io;
use std::io::ErrorKind;
use std::sync::Arc;
use uuid::Uuid;

pub struct ProductService;

#[async_trait]
impl interfaces::Service<Product, Description> for ProductService {
    type Error = Box<dyn Error + Send + Sync>;
    type Repository = Arc<dyn Repository<Product, Error = Self::Error, Id = Uuid> + Send + Sync>;
    type MessageBroker =
        Arc<dyn MessageBroker<dyn Identifiable + Send + Sync, Error = Self::Error> + Send + Sync>;

    async fn create(
        &self,
        dto: Description,
        repo: Self::Repository,
        broker: Self::MessageBroker,
    ) -> Result<Product, Self::Error> {
        let id = Uuid::new_v4();
        let product = Product {
            id,
            name: dto.name,
            price: dto.price,
        };
        repo.add(product.clone()).await?;
        broker.send("product-events", "create", &product).await?;
        Ok(product)
    }

    async fn update(
        &self,
        id: Uuid,
        dto: Description,
        repo: Self::Repository,
        broker: Self::MessageBroker,
    ) -> Result<(), Self::Error> {
        let product = repo.get(id).await?;
        if let Some(mut product) = product {
            product.name = dto.name;
            product.price = dto.price;
            repo.update(product.clone()).await?;
            broker.send("product-events", "update", &product).await
        } else {
            Err(io::Error::from(ErrorKind::NotFound))?
        }
    }

    async fn delete(
        &self,
        id: Uuid,
        repo: Self::Repository,
        broker: Self::MessageBroker,
    ) -> Result<(), Self::Error> {
        repo.remove(id).await?;
        broker.send("product-events", "delete", &id).await
    }
}
