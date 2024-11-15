use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::interfaces::Identifiable;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

impl Identifiable for User {
    fn id(&self) -> String {
        self.id.to_string()
    }
}
