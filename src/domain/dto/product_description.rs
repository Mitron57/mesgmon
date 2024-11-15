use serde::Deserialize;

#[derive(Deserialize)]
pub struct Description {
    pub name: String,
    pub price: usize,
}