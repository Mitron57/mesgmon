use axum::async_trait;

#[async_trait]
pub trait MessageBroker<M: ?Sized> {
    type Error;
    async fn send(&self, topic: &str, action: &str, message: &M) -> Result<(), Self::Error>;
}
