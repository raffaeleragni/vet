use std::time::Duration;

use cucumber::gherkin::Step;
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

struct TopicMetadata {
    partitions: usize,
    replicas: usize,
    min_isr: usize,
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

    fn topic_metadata(&self, topic: &str) -> TopicMetadata {
        let consumer = self.consumer();
        let metadata = consumer
            .fetch_metadata(Some(topic), Duration::from_secs(1))
            .expect("Topic not found");
        let topic_metadata = metadata.topics().iter().next().expect("Topic not found");
        let partitions = topic_metadata.partitions();
        let partition_count = partitions.len();
        let min_isr = partitions
            .iter()
            .map(|p| p.isr().iter().min().unwrap())
            .min()
            .unwrap()
            .to_owned() as usize;
        let replicas = partitions
            .iter()
            .map(|p| p.replicas().iter().min().unwrap())
            .min()
            .unwrap()
            .to_owned() as usize;
        TopicMetadata {
            partitions: partition_count,
            replicas,
            min_isr,
        }
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
    env.kafka.topic_metadata(topic.as_str());
}

#[then(expr = "kafka topic {string} exists with settings:")]
async fn then_topic_exists_with_settings(env: &mut WorldEnv, topic: String, step: &Step) {
    let metadata = env.kafka.topic_metadata(topic.as_str());
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter() {
            let key = &row[0];
            let value = &row[1];
            match key.as_str().trim() {
                "Min ISR" => assert!(value.parse::<usize>().unwrap() <= metadata.min_isr),
                "Replicas" => assert_eq!(metadata.replicas, value.parse::<usize>().unwrap()),
                "Partitions" => assert_eq!(metadata.partitions, value.parse::<usize>().unwrap()),
                _ => {}
            }
        }
    }
}
