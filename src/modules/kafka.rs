use std::time::Duration;

use cucumber::{given, then, when};
use rdkafka::client::DefaultClientContext;
use rdkafka::consumer::{BaseConsumer, Consumer};

use crate::WorldEnv;

use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::config::ClientConfig;

#[derive(Debug, Default)]
pub struct Env {
    pub brokers: String,
}

impl Env {
    fn admin_client(&self) -> AdminClient<DefaultClientContext> {
        ClientConfig::new()
            .set("bootstrap.servers", self.brokers.as_str())
            .set("session.timeout.ms", "5000")
            .create()
            .expect("Admin client creation failed")
    }

    fn consumer(&self) -> BaseConsumer {
        ClientConfig::new()
            .set("bootstrap.servers", self.brokers.as_str())
            .set("session.timeout.ms", "5000")
            .create()
            .expect("Consumer creation failed")
    }
}

#[given(expr = "kafka broker is {string}")]
async fn given_kafka_broker(env: &mut WorldEnv, brokers: String) {
    env.kafka.brokers = brokers;
}

#[when(expr = "kafka create topic {string}")]
async fn when_create_topic(env: &mut WorldEnv, topic: String) {
    let admin = env.kafka.admin_client();
    let opts = AdminOptions::new().operation_timeout(Some(Duration::from_secs(1)));
    let new_topic = NewTopic::new(topic.as_str(), 1, TopicReplication::Fixed(1));
    admin
        .create_topics(&[new_topic], &opts)
        .await
        .expect("Topic creation failed");
}

#[then(expr = "kafka topic {string} exists")]
async fn then_topic_exists(env: &mut WorldEnv, topic: String) {
    let consumer = env.kafka.consumer();
    consumer
        .fetch_metadata(Some(topic.as_str()), Duration::from_secs(1))
        .expect("Topic not found");
}
