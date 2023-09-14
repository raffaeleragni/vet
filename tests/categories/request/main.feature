Feature: Health check
  Scenario: Health check
    When health, a get request to 'http://localhost:8080/status'
    Then health status is 200
  Scenario: Health check POST
    When health, a post request to 'http://localhost:8080/status'
    Then health status is 201
