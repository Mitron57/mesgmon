use crate::domain::dto::{Credentials, Description};
use crate::domain::interfaces::{Identifiable, MessageBroker, Repository, Service};
use crate::domain::models::{Product, User};
use std::error;
use std::sync::Arc;
use uuid::Uuid;

type Error = Box<dyn error::Error + Send + Sync>;
type Id = dyn Identifiable + Sync + Send;
type Broker = Arc<dyn MessageBroker<Id, Error = Error> + Sync + Send>;

type UserRepo = Arc<dyn Repository<User, Error = Error, Id = Uuid> + Sync + Send>;
type UserService = dyn Service<User, Credentials, Error = Error, MessageBroker = Broker, Repository = UserRepo>
    + Send
    + Sync;

type ProductRepo = Arc<dyn Repository<Product, Error = Error, Id = Uuid> + Sync + Send>;
type ProductService = dyn Service<Product, Description, Error = Error, MessageBroker = Broker, Repository = ProductRepo>
    + Send
    + Sync;

pub struct AppState {
    pub broker: Broker,
    pub user_repo: UserRepo,
    pub product_repo: ProductRepo,
    pub user_service: Arc<UserService>,
    pub product_service: Arc<ProductService>,
}
