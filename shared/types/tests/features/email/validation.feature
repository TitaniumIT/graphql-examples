Feature: Validate email address

Scenario Outline: Correct Emails
    Given Email <email>
    When Created
    Then Validation should not fail

Examples:
    | email          |
    | email@email.nl |
