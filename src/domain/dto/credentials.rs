use serde::Deserialize;

#[derive(Deserialize)]
pub struct Credentials {
    pub name: String,
    pub email: String,
}