# Feature `kafka`

# Given setups

- `Given kafka broker is {string}`

  Example:

  ```gherkin
  Given kafka broker is "localhost:9092"
  ```

# When actions

- `When kafka create topic {string}`
  
  Example:

  ```gherkin
  When kafka create topic "test-topic"
  ```

# Then assertions

- `Then kafka topic {string} exists`
  
  Example:

  ```gherkin
  Then kafka topic "test-topic" exists
  ```


- `Then kafka topic {string} exists with settings:`

  Example:

  ```gherkin
  Then kafka topic "test-topic" exists with settings:
      | Partitions | 1 |
      | Replicas   | 1 |
      | Min ISR    | 1 |
  ```
