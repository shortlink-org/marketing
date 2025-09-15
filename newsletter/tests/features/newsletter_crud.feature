Feature: Newsletter CRUD Operations
  As a newsletter service
  I want to perform CRUD operations on newsletter subscriptions
  So that users can manage their newsletter subscriptions

  Background:
    Given the newsletter service is running
    And the database is clean

  Scenario: Create a new newsletter subscription
    When I subscribe email "test@example.com"
    Then the subscription should be created successfully
    And the email "test@example.com" should be active

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

  Scenario: List all newsletter subscriptions
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

  Scenario: Update subscription status to inactive (bulk)
    Given I have subscribed email "update1@example.com"
    And I have subscribed email "update2@example.com"
    When I unsubscribe email "update1@example.com"
    And I unsubscribe email "update2@example.com"
    Then the operation should complete successfully
    And "update1@example.com" should not be active
    And "update2@example.com" should not be active

  Scenario: Update subscription status to active (bulk)
    When I subscribe email "reactive1@example.com"
    And I subscribe email "reactive2@example.com"
    Then the subscription should be created successfully
    And "reactive1@example.com" should be active
    And "reactive2@example.com" should be active

  Scenario: Delete a single newsletter subscription
    Given I have subscribed email "delete-me@example.com"
    When I unsubscribe email "delete-me@example.com"
    Then the subscription should be deleted successfully
    And the email "delete-me@example.com" should not exist

  Scenario: Delete non-existent newsletter subscription
    When I unsubscribe email "not-exists@example.com"
    Then the operation should complete successfully

  Scenario: Delete multiple newsletter subscriptions (bulk)
    Given I have subscribed email "bulk1@example.com"
    And I have subscribed email "bulk2@example.com"
    And I have subscribed email "bulk3@example.com"
    When I unsubscribe email "bulk1@example.com"
    And I unsubscribe email "bulk2@example.com"
    Then the operation should complete successfully
    And the email "bulk1@example.com" should not exist
    And the email "bulk2@example.com" should not exist
    And the email "bulk3@example.com" should still exist

  Scenario: Complex workflow - Subscribe, Update, and Delete
    When I subscribe email "workflow@example.com"
    Then the email "workflow@example.com" should be active
    When I unsubscribe email "workflow@example.com"
    Then "workflow@example.com" should not be active
    When I subscribe email "workflow@example.com"
    Then "workflow@example.com" should be active
    When I unsubscribe email "workflow@example.com"
    Then the email "workflow@example.com" should not exist