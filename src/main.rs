use crate::application::{AppState, ProductService, UserService};
use crate::handlers::{product, user};
use crate::infrastructure::{Kafka, Postgres, Repository};
use axum::routing::{post, put};
use axum::Router;
use std::error::Error;
use std::sync::Arc;
use log::info;

mod application;
mod domain;
mod handlers;
mod infrastructure;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    if dotenvy::dotenv().is_err() {
        println!("Failed to load .env file, using manually specified env variables...");
    }
    env_logger::init();
    
    let database_uri = std::env::var("DATABASE_URI")?;
    let kafka_brokers = std::env::var("KAFKA_BROKERS")?;
    let hostaddr = std::env::var("HOSTADDR")?;
    
    let postgres = Arc::new(Postgres::new(&database_uri).await?);
    let user_repo = Arc::new(Repository {
        storage: postgres.clone(),
    });
    let product_repo = Arc::new(Repository { storage: postgres });
    let kafka = Arc::new(Kafka::new(&kafka_brokers)?);
    let state = Arc::new(AppState {
        broker: kafka,
        user_repo,
        product_repo,
        user_service: Arc::new(UserService),
        product_service: Arc::new(ProductService),
    });
    
    let user = Router::new()
        .route("/users", post(user::create))
        .route("/users/:id", put(user::update).delete(user::delete))
        .with_state(state.clone());
    let product = Router::new()
        .route("/products", post(product::create))
        .route(
            "/products/:id",
            put(product::update).delete(product::delete),
        )
        .with_state(state);
    
    let app = Router::merge(user, product);
    let listener = tokio::net::TcpListener::bind(&hostaddr).await?;
    info!("Listening on: {}", hostaddr);
    axum::serve(listener, app).await?;
    Ok(())
}
