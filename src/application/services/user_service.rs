use crate::domain::dto::Credentials;
use crate::domain::interfaces;
use crate::domain::interfaces::{Identifiable, MessageBroker, Repository};
use crate::domain::models::User;
use axum::async_trait;
use std::error::Error;
use std::io;
use std::io::ErrorKind;
use std::sync::Arc;
use uuid::Uuid;

pub struct UserService;

#[async_trait]
impl interfaces::Service<User, Credentials> for UserService {
    type Error = Box<dyn Error + Send + Sync>;
    type Repository = Arc<dyn Repository<User, Error = Self::Error, Id = Uuid> + Send + Sync>;
    type MessageBroker = Arc<dyn MessageBroker<dyn Identifiable + Send + Sync, Error = Self::Error> + Send + Sync>;

    async fn create(
        &self,
        dto: Credentials,
        repo: Self::Repository,
        broker: Self::MessageBroker,
    ) -> Result<User, Self::Error> {
        let id = Uuid::new_v4();
        let user = User {
            id,
            name: dto.name,
            email: dto.email,
        };
        repo.add(user.clone()).await?;
        broker.send("user-events", "create", &user).await?;
        Ok(user)
    }

    async fn update(
        &self,
        id: Uuid,
        dto: Credentials,
        repo: Self::Repository,
        broker: Self::MessageBroker,
    ) -> Result<(), Self::Error> {
        let user = repo.get(id).await?;
        if let Some(mut user) = user {
            user.name = dto.name;
            user.email = dto.email;
            repo.update(user.clone()).await?;
            broker.send("user-events", "update", &user).await
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
        broker.send("user-events", "delete", &id).await
    }
}
