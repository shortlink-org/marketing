Feature: Newsletter CRUD Operations with Cucumber Expressions
  As a newsletter service
  I want to perform CRUD operations on newsletter subscriptions
  So that users can manage their newsletter subscriptions using expressive test scenarios

  Background:
    Given the newsletter service is running
    And the database is clean

  Scenario: Create a new newsletter subscription using string parameter
    When I subscribe email "user@example.com"
    Then the subscription should be created successfully
    And the email "user@example.com" should be active

  Scenario: Create duplicate newsletter subscription
    Given I have subscribed email "duplicate@example.com"
    When I subscribe email "duplicate@example.com" again
    Then the subscription should be created successfully
    And there should be only one subscription for "duplicate@example.com"

  Scenario: Read a single newsletter subscription
    Given I have subscribed email "reader@example.com"
    When I get the subscription for "reader@example.com"
    Then the subscription should exist
    And the email should be "reader@example.com"
    And the status should be active

  Scenario: Read non-existent newsletter subscription
    When I get the subscription for "nonexistent@example.com"
    Then the subscription should not exist
    And the status should be inactive

  Scenario: List subscriptions using integer parameter
    Given I have subscribed email "list1@example.com"
    And I have subscribed email "list2@example.com"
    And I have subscribed email "list3@example.com"
    When I list all subscriptions
    Then I should get 3 subscriptions
    And the list should contain "list1@example.com"
    And the list should contain "list2@example.com"
    And the list should contain "list3@example.com"

  Scenario: List subscriptions when none exist
    When I list all subscriptions
    Then I should get 0 subscriptions

  Scenario: Delete single subscription
    Given I have subscribed email "delete-me@example.com"
    When I unsubscribe email "delete-me@example.com"
    Then the subscription should be deleted successfully
    And the email "delete-me@example.com" should not exist

  Scenario: Delete non-existent subscription
    When I unsubscribe email "not-exists@example.com"
    Then the operation should complete successfully

  Scenario: Complex workflow - Subscribe, Update, and Delete
    When I subscribe email "workflow@example.com"
    Then the email "workflow@example.com" should be active
    When I unsubscribe email "workflow@example.com"
    Then "workflow@example.com" should not be active
    When I subscribe email "workflow@example.com"
    Then "workflow@example.com" should be active
    When I unsubscribe email "workflow@example.com"
    Then the email "workflow@example.com" should not exist

  Scenario: Bulk subscription with integer and string parameters
    When I subscribe 5 emails with domain "testdomain.com"
    Then I should get 5 subscriptions
    And there should be 5 active subscriptions

  Scenario: Bulk operations by domain
    Given I have subscribed email "user1@bulk.com"
    And I have subscribed email "user2@bulk.com"
    And I have subscribed email "user3@keep.com"
    When I perform bulk unsubscribe for domain "bulk.com"
    Then the operation should complete successfully
    And no emails with domain "bulk.com" should exist
    And the email "user3@keep.com" should still exist

  Scenario: Email validation
    When I subscribe email "valid@example.com"
    Then the email "valid@example.com" should be valid
    And the subscription should be created successfully

  Scenario Outline: Multiple email subscriptions with examples
    When I subscribe email "<email>"
    Then the subscription should be created successfully
    And the email "<email>" should be active
    And the email "<email>" should be valid

    Examples:
      | email                |
      | test1@example.com    |
      | test2@example.org    |
      | user@testdomain.net  |

  Scenario: Advanced bulk operations with mixed parameters
    When I subscribe 3 emails with domain "company.com"
    And I subscribe email "external@other.com"
    Then I should get 4 subscriptions
    When I perform bulk unsubscribe for domain "company.com"
    Then I should get 1 subscriptions
    And the email "external@other.com" should still exist