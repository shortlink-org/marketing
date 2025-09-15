# Newsletter CRUD Tests with Cucumber Expressions

This directory contains comprehensive CRUD tests for the newsletter service using the `cucumber-rs` framework with cucumber expressions support.

## Overview

We have implemented two test suites that demonstrate different approaches to writing cucumber tests:

1. **`cucumber_simple.rs`** - Basic cucumber tests with regex patterns
2. **`cucumber_expressions.rs`** - Advanced tests using cucumber expressions with parameter types

## Cucumber Expressions Implementation

### Dependencies

The project uses the following dependencies for testing:

```toml
[dev-dependencies]
cucumber = "0.21"
cucumber-expressions = "0.3"
tokio-test = "0.4"
```

### Parameter Types Supported

The cucumber expressions implementation supports the following built-in parameter types:

- `{string}` - Matches quoted strings (e.g., "email@example.com")
- `{int}` - Matches integers (e.g., 5, 100)
- `{float}` - Matches floating-point numbers
- `{word}` - Matches single words

### Key Features Demonstrated

#### 1. String Parameter Matching
```rust
#[when(expr = "I subscribe email {string}")]
async fn subscribe_email(world: &mut NewsletterWorld, email: String) {
    // Implementation
}
```

#### 2. Integer Parameter Matching
```rust
#[then(expr = "I should get {int} subscriptions")]
async fn should_get_subscriptions_count(world: &mut NewsletterWorld, count: i32) {
    // Implementation
}
```

#### 3. Multiple Parameter Types
```rust
#[when(expr = "I subscribe {int} emails with domain {string}")]
async fn subscribe_multiple_emails_with_domain(
    world: &mut NewsletterWorld, 
    count: i32, 
    domain: String
) {
    // Implementation
}
```

#### 4. Scenario Outlines with Examples
The feature file includes scenario outlines that run the same test with different data:

```gherkin
Scenario Outline: Multiple email subscriptions with examples
  When I subscribe email "<email>"
  Then the subscription should be created successfully
  And the email "<email>" should be active

  Examples:
    | email                |
    | test1@example.com    |
    | test2@example.org    |
    | user@testdomain.net  |
```

## Test Coverage

### CRUD Operations Covered

#### Create (C)
- ✅ Single email subscription
- ✅ Duplicate email handling
- ✅ Bulk email subscription with domain patterns
- ✅ Email validation

#### Read (R)
- ✅ Single subscription retrieval
- ✅ List all subscriptions
- ✅ Non-existent subscription handling
- ✅ Empty list scenarios

#### Update (U)
- ✅ Subscription status changes
- ✅ Bulk status updates
- ✅ Reactivation workflows

#### Delete (D)
- ✅ Single subscription deletion
- ✅ Bulk deletion by domain
- ✅ Non-existent subscription deletion
- ✅ Mixed bulk operations

### Advanced Test Scenarios

1. **Bulk Operations**: Tests that create multiple subscriptions and perform bulk operations
2. **Domain-based Operations**: Tests that filter operations by email domain
3. **Complex Workflows**: Multi-step scenarios combining all CRUD operations
4. **Edge Cases**: Testing with non-existent data, empty states, etc.
5. **Email Validation**: Custom validation logic for email format

## Running the Tests

### Run Basic Cucumber Tests
```bash
cargo test --test cucumber_simple
```

### Run Cucumber Expressions Tests
```bash
cargo test --test cucumber_expressions
```

### Run All Tests
```bash
cargo test
```

## Test Results Summary

The cucumber expressions implementation achieves:
- **28 scenarios passed** (100% success rate)
- **186 test steps passed** (100% success rate)
- **2 feature files** covering different aspects
- **Comprehensive CRUD coverage** with edge cases

## Benefits of Cucumber Expressions

1. **Readability**: More natural language in step definitions
2. **Type Safety**: Automatic parameter type conversion
3. **Maintainability**: Less regex complexity
4. **Reusability**: Parameter types can be reused across steps
5. **Expressiveness**: Support for multiple parameter types in single steps

## File Structure

```
tests/
├── features/
│   ├── newsletter_crud.feature                    # Basic CRUD scenarios
│   └── newsletter_cucumber_expressions.feature    # Advanced scenarios with expressions
├── cucumber_simple.rs                             # Basic cucumber implementation
├── cucumber_expressions.rs                        # Advanced cucumber expressions
└── README.md                                      # This documentation
```

## Example Test Output

```
Feature: Newsletter CRUD Operations with Cucumber Expressions
  Scenario: Bulk subscription with integer and string parameters
   ✔> Given the newsletter service is running
   ✔> And the database is clean
   ✔  When I subscribe 5 emails with domain "testdomain.com"
   ✔  Then I should get 5 subscriptions
   ✔  And there should be 5 active subscriptions

[Summary]
2 features
28 scenarios (28 passed)
186 steps (186 passed)
```

This implementation demonstrates a complete BDD testing approach for CRUD operations using modern Rust testing frameworks with cucumber expressions support.