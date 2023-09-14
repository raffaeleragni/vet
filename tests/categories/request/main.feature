Feature: Requests
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
  Scenario: PUT
    When y, a put request to 'http://localhost:8080/put0'
    Then y status is 200
  Scenario: PUT with body
    When y, a put request with body to 'http://localhost:8080/put1'
      """
      {}
      """
    Then y status is 201
  Scenario: DELETE
    When x, a delete request to 'http://localhost:8080/delete'
    Then x status is 200
  Scenario: HEAD
    When x, a head request to 'http://localhost:8080/head'
    Then x status is 200
