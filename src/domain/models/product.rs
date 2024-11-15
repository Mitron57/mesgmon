use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::domain::interfaces::Identifiable;

#[derive(Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub price: usize,
}

impl Identifiable for Product {
    fn id(&self) -> String {
        self.id.to_string()
    }
}