# Feature: `request`

# Given setups

- `Given next request headers will be` + DATATABLE
  
  Needs to set headers again every time a request happens.
  For setting it for all requests, see "Given all requsts headers will be"

  Example:

  ```gherkin
  Given next request headers will be
    | X-Custom-Header | custom value |
  ```

- `Given all requests headers will be` + DATATABLE

  Example:

  ```gherkin
  Given all requests headers will be
    | Authorization | bearer XXX |
  ```
  
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

