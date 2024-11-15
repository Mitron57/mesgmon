use uuid::Uuid;

pub trait Identifiable {
    fn id(&self) -> String;
}

impl Identifiable for Uuid {
    fn id(&self) -> String {
        self.to_string()
    }
}