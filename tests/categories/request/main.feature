Feature: Health check
  Scenario: Health check
    When health, a get request to 'http://localhost:8080/status0'
    Then health status is 200
  Scenario: Health check POST
    When health, a post request to 'http://localhost:8080/status1'
    Then health status is 201
  Scenario: Health check POST with body
    When health, a post request with body to 'http://localhost:8080/status2'
      """
      {}
      """
    Then health status is 202
