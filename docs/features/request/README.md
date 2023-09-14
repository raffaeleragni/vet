# Feature: `request`

# When actions

- `When <identifier>, a <method> request to '<URL>'`

  Example:

  ```gherkin
  When health, a get request to 'http://localhost:8080/status'
  ```

- `When <identifier>, a <method> with body request to '<URL>'` + DOCSTRING
  
  Example:

  ```gherkin
  When health, a post request with body to 'http://localhost:8080/status2'
    """
    {}
    """
  ```

# Then assertions

- `Then <identifier> status is <status code>`

  Example:

  ```gherkin
  Then health status is 200
  ```

