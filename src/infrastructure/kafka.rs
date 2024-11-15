use crate::domain::interfaces;
use crate::domain::interfaces::Identifiable;
use axum::async_trait;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;

pub struct Kafka {
    producer: FutureProducer,
}

impl Kafka {
    pub fn new(kafka_brokers: &str) -> Result<Kafka, Box<dyn Error + Send + Sync>> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", kafka_brokers)
            .create()?;
        Ok(Kafka { producer })
    }
}

#[async_trait]
impl<M: Identifiable + Sync + ?Sized> interfaces::MessageBroker<M> for Kafka {
    type Error = Box<dyn Error + Send + Sync>;

    async fn send(&self, topic: &str, action: &str, message: &M) -> Result<(), Self::Error> {
        let payload = HashMap::from([
            ("topic", topic.to_string()),
            ("action", action.to_string()),
            ("entity_id", message.id()),
        ]);
        let result = self
            .producer
            .send(
                FutureRecord::to(topic)
                    .payload(&serde_json::to_vec(&payload)?)
                    .key(action),
                Duration::from_secs(1),
            )
            .await;
        if let Err(err) = result {
            return Err(err.0.into());
        }
        Ok(())
    }
}
