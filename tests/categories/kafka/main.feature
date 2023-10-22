Feature: Kafka
  Scenario: Topic exists
    Given kafka broker is "localhost:9092"
    When kafka create topic "test-topic"
    Then kafka topic "test-topic" exists
    Then kafka topic "test-topic" exists with settings:
      | Partitions | 1 |
      | Replicas   | 1 |
      | Min ISR    | 1 |
  Scenario: Message without key
    Given kafka broker is "localhost:9092"
    Given kafka has topic "test-topic"
    When kafka topic "test-topic" message sent:
      """
      {
        "name": "message",
        "priority": 5
        }
      """
    Then kafka topic "test-topic" contains:
      """
      {
        "name": "message",
        "priority": 5
      }
      """
