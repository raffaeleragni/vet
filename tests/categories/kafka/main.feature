Feature: Kafka
  Scenario: Topic exists
    Given kafka broker is "localhost:9092"
    When kafka create topic "test-topic"
    Then kafka topic "test-topic" exists
    Then kafka topic "test-topic" exists with settings:
      | Partitions | 1 |
      | Replicas   | 1 |
      | Min ISR    | 1 |
