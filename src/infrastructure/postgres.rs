use crate::domain::interfaces;
use crate::domain::models::{Product, User};
use axum::async_trait;
use deadpool_postgres::{Manager, Pool, Transaction};
use std::error;
use std::str::FromStr;
use tokio_postgres::{Config, NoTls};
use uuid::Uuid;

type Error = Box<dyn error::Error + Send + Sync>;

pub struct Postgres {
    pool: Pool,
}

impl Postgres {
    pub async fn new(database_uri: &str) -> Result<Postgres, Error> {
        let config = Config::from_str(database_uri)?;
        let manager = Manager::new(config, NoTls);
        let pool = Pool::builder(manager).build()?;
        Ok(Postgres { pool })
    }

    pub async fn commit_or_rollback<T>(
        transaction: Transaction<'_>,
        result: &Result<T, tokio_postgres::Error>,
    ) -> Result<(), Error> {
        if result.is_err() {
            transaction.rollback().await?;
        } else {
            transaction.commit().await?;
        }
        Ok(())
    }
}

#[async_trait]
impl interfaces::Database<User> for Postgres {
    type Error = Error;

    async fn add(&self, item: User) -> Result<(), Self::Error> {
        let mut connection = self.pool.get().await?;
        let statement = connection
            .prepare_cached("INSERT INTO Users (id, name, email) VALUES ($1, $2, $3)")
            .await?;
        let transaction = connection.transaction().await?;
        let result = transaction
            .execute(&statement, &[&item.id.to_string(), &item.name, &item.email])
            .await;
        Self::commit_or_rollback(transaction, &result).await
    }

    async fn delete(&self, id: Uuid) -> Result<(), Self::Error> {
        let mut connection = self.pool.get().await?;
        let statement = connection
            .prepare_cached("DELETE FROM Users WHERE id = $1")
            .await?;
        let transaction = connection.transaction().await?;
        let result = transaction.execute(&statement, &[&id.to_string()]).await;
        Self::commit_or_rollback(transaction, &result).await
    }

    async fn update(&self, item: User) -> Result<(), Self::Error> {
        let mut connection = self.pool.get().await?;
        let statement = connection
            .prepare_cached("UPDATE Users SET name = $1, email = $2 WHERE id = $3")
            .await?;
        let transaction = connection.transaction().await?;
        let result = transaction
            .execute(&statement, &[&item.name, &item.email, &item.id.to_string()])
            .await;
        Self::commit_or_rollback(transaction, &result).await
    }

    async fn get(&self, id: Uuid) -> Result<Option<User>, Self::Error> {
        let mut connection = self.pool.get().await?;
        let statement = connection
            .prepare_cached("SELECT (name, email) FROM Users WHERE id = $1")
            .await?;
        let transaction = connection.transaction().await?;
        let result = transaction.query(&statement, &[&id.to_string()]).await;
        if result.is_err() {
            Self::commit_or_rollback(transaction, &result).await?;
            return Err(result.unwrap_err().into());
        }
        transaction.commit().await?;
        let row = &result.unwrap()[0];
        match (row.get::<_, String>("name"), row.get::<_, String>("email")) {
            (name, email) if !name.is_empty() && !email.is_empty() => {
                let user = User { id, name, email };
                Ok(Some(user))
            }
            _ => Ok(None),
        }
    }
}

#[async_trait]
impl interfaces::Database<Product> for Postgres {
    type Error = Error;

    async fn add(&self, item: Product) -> Result<(), Self::Error> {
        let mut connection = self.pool.get().await?;
        let statement = connection
            .prepare_cached("INSERT INTO Products (id, name, price) VALUES ($1, $2, $3)")
            .await?;
        let transaction = connection.transaction().await?;
        let result = transaction
            .execute(
                &statement,
                &[&item.id.to_string(), &item.name, &(item.price as i64)],
            )
            .await;
        Self::commit_or_rollback(transaction, &result).await
    }

    async fn delete(&self, id: Uuid) -> Result<(), Self::Error> {
        let mut connection = self.pool.get().await?;
        let statement = connection
            .prepare_cached("DELETE FROM Products WHERE id = $1")
            .await?;
        let transaction = connection.transaction().await?;
        let result = transaction.execute(&statement, &[&id.to_string()]).await;
        Self::commit_or_rollback(transaction, &result).await
    }

    async fn update(&self, item: Product) -> Result<(), Self::Error> {
        let mut connection = self.pool.get().await?;
        let statement = connection
            .prepare_cached("UPDATE Products SET name = $1, price = $2 WHERE id = $3")
            .await?;
        let transaction = connection.transaction().await?;
        let result = transaction
            .execute(
                &statement,
                &[&item.name, &(item.price as i64), &item.id.to_string()],
            )
            .await;
        Self::commit_or_rollback(transaction, &result).await
    }

    async fn get(&self, id: Uuid) -> Result<Option<Product>, Self::Error> {
        let mut connection = self.pool.get().await?;
        let statement = connection
            .prepare_cached("SELECT (name, price) FROM Products WHERE id = $1")
            .await?;
        let transaction = connection.transaction().await?;
        let result = transaction.query(&statement, &[&id.to_string()]).await;
        if result.is_err() {
            Self::commit_or_rollback(transaction, &result).await?;
            return Err(result.unwrap_err().into());
        }
        transaction.commit().await?;
        let row = &result.unwrap()[0];
        match (row.get::<_, String>("name"), row.get::<_, i64>("price")) {
            (name, price) if !name.is_empty() => {
                let product = Product {
                    id,
                    name,
                    price: price as usize,
                };
                Ok(Some(product))
            }
            _ => Ok(None),
        }
    }
}
