use cucumber::{given, then, when, World};
use std::collections::HashMap;

// Simple in-memory newsletter store for testing
#[derive(Debug, Clone)]
pub struct Newsletter {
    pub email: String,
    pub active: bool,
}

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct NewsletterWorld {
    pub newsletters: HashMap<String, Newsletter>,
    pub last_response: Option<String>,
    pub last_list: Vec<Newsletter>,
    pub last_get: Option<Newsletter>,
}

impl Default for NewsletterWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl NewsletterWorld {
    pub fn new() -> Self {
        Self {
            newsletters: HashMap::new(),
            last_response: None,
            last_list: Vec::new(),
            last_get: None,
        }
    }

    pub fn cleanup(&mut self) {
        self.newsletters.clear();
        self.last_response = None;
        self.last_list.clear();
        self.last_get = None;
    }
}

// Background steps
#[given("the newsletter service is running")]
async fn service_is_running(world: &mut NewsletterWorld) {
    world.last_response = Some("service_running".to_string());
}

#[given("the database is clean")]
async fn database_is_clean(world: &mut NewsletterWorld) {
    world.cleanup();
}

// Create operations using cucumber expressions
#[when(expr = "I subscribe email {string}")]
async fn subscribe_email(world: &mut NewsletterWorld, email: String) {
    let newsletter = Newsletter {
        email: email.clone(),
        active: true,
    };
    world.newsletters.insert(email, newsletter);
    world.last_response = Some("success".to_string());
}

#[given(expr = "I have subscribed email {string}")]
async fn have_subscribed_email(world: &mut NewsletterWorld, email: String) {
    subscribe_email(world, email).await;
}

#[when(expr = "I subscribe email {string} again")]
async fn subscribe_email_again(world: &mut NewsletterWorld, email: String) {
    // This should not create a duplicate - just update existing
    subscribe_email(world, email).await;
}

// Read operations using cucumber expressions
#[when(expr = "I get the subscription for {string}")]
async fn get_subscription(world: &mut NewsletterWorld, email: String) {
    world.last_get = world.newsletters.get(&email).cloned();
}

#[when("I list all subscriptions")]
async fn list_all_subscriptions(world: &mut NewsletterWorld) {
    world.last_list = world.newsletters.values().cloned().collect();
}

// Delete operations using cucumber expressions
#[when(expr = "I unsubscribe email {string}")]
async fn unsubscribe_email(world: &mut NewsletterWorld, email: String) {
    world.newsletters.remove(&email);
    world.last_response = Some("success".to_string());
}

// Assertion steps using cucumber expressions
#[then("the subscription should be created successfully")]
async fn subscription_created_successfully(world: &mut NewsletterWorld) {
    assert_eq!(world.last_response, Some("success".to_string()));
}

#[then("the operation should complete successfully")]
async fn operation_completed_successfully(world: &mut NewsletterWorld) {
    assert_eq!(world.last_response, Some("success".to_string()));
}

#[then("the subscription should be deleted successfully")]
async fn subscription_deleted_successfully(world: &mut NewsletterWorld) {
    assert_eq!(world.last_response, Some("success".to_string()));
}

#[then("the subscriptions should be deleted successfully")]
async fn subscriptions_deleted_successfully(world: &mut NewsletterWorld) {
    assert_eq!(world.last_response, Some("success".to_string()));
}

#[then(expr = "the email {string} should be active")]
async fn email_should_be_active(world: &mut NewsletterWorld, email: String) {
    let newsletter = world.newsletters.get(&email);
    assert!(newsletter.is_some(), "Email {} should exist", email);
    assert!(newsletter.unwrap().active, "Email {} should be active", email);
}

#[then(expr = "{string} should be active")]
async fn email_active(world: &mut NewsletterWorld, email: String) {
    email_should_be_active(world, email).await;
}

#[then(expr = "{string} should not be active")]
async fn email_should_not_be_active(world: &mut NewsletterWorld, email: String) {
    let newsletter = world.newsletters.get(&email);
    // Email should either not exist or be inactive
    assert!(newsletter.is_none(), "Email {} should not exist (should be deleted)", email);
}

#[then(expr = "the email {string} should not exist")]
async fn email_should_not_exist(world: &mut NewsletterWorld, email: String) {
    let newsletter = world.newsletters.get(&email);
    assert!(newsletter.is_none(), "Email {} should not exist", email);
}

#[then(expr = "the email {string} should still exist")]
async fn email_should_still_exist(world: &mut NewsletterWorld, email: String) {
    let newsletter = world.newsletters.get(&email);
    assert!(newsletter.is_some(), "Email {} should still exist", email);
}

#[then(expr = "there should be only one subscription for {string}")]
async fn only_one_subscription(world: &mut NewsletterWorld, email: String) {
    let count = world.newsletters.values().filter(|n| n.email == email).count();
    assert_eq!(count, 1, "Should have exactly one subscription for {}", email);
}

#[then("the subscription should exist")]
async fn subscription_should_exist(world: &mut NewsletterWorld) {
    assert!(world.last_get.is_some(), "Should have found a subscription");
    assert!(world.last_get.as_ref().unwrap().active, "Subscription should be active");
}

#[then("the subscription should not exist")]
async fn subscription_should_not_exist(world: &mut NewsletterWorld) {
    assert!(world.last_get.is_none(), "Should not have found a subscription");
}

#[then(expr = "the email should be {string}")]
async fn email_should_be(world: &mut NewsletterWorld, expected_email: String) {
    assert!(world.last_get.is_some(), "Should have a get response");
    let newsletter = world.last_get.as_ref().unwrap();
    assert_eq!(newsletter.email, expected_email, "Email should match expected value");
}

#[then("the status should be active")]
async fn status_should_be_active(world: &mut NewsletterWorld) {
    assert!(world.last_get.is_some(), "Should have a get response");
    let newsletter = world.last_get.as_ref().unwrap();
    assert!(newsletter.active, "Status should be active");
}

#[then("the status should be inactive")]
async fn status_should_be_inactive(world: &mut NewsletterWorld) {
    // In our simple model, inactive means not found
    assert!(world.last_get.is_none(), "Status should be inactive (not found)");
}

// Using {int} parameter type for numeric values
#[then(expr = "I should get {int} subscriptions")]
async fn should_get_subscriptions_count(world: &mut NewsletterWorld, count: i32) {
    // First, populate the list with current subscriptions
    world.last_list = world.newsletters.values().cloned().collect();
    assert_eq!(world.last_list.len(), count as usize, "Should have {} subscriptions", count);
}

#[then(expr = "the list should contain {string}")]
async fn list_should_contain_email(world: &mut NewsletterWorld, email: String) {
    let found = world.last_list.iter().any(|n| n.email == email);
    assert!(found, "List should contain email {}", email);
}

// Advanced cucumber expressions with multiple parameters
#[when(expr = "I subscribe {int} emails with domain {string}")]
async fn subscribe_multiple_emails_with_domain(world: &mut NewsletterWorld, count: i32, domain: String) {
    for i in 1..=count {
        let email = format!("user{}@{}", i, domain);
        subscribe_email(world, email).await;
    }
}

#[then(expr = "there should be {int} active subscriptions")]
async fn should_have_active_subscriptions(world: &mut NewsletterWorld, count: i32) {
    let active_count = world.newsletters.values().filter(|n| n.active).count();
    assert_eq!(active_count, count as usize, "Should have {} active subscriptions", count);
}

// Custom step for bulk operations
#[when(expr = "I perform bulk unsubscribe for domain {string}")]
async fn bulk_unsubscribe_by_domain(world: &mut NewsletterWorld, domain: String) {
    let emails_to_remove: Vec<String> = world
        .newsletters
        .keys()
        .filter(|email| email.contains(&format!("@{}", domain)))
        .cloned()
        .collect();
    
    for email in emails_to_remove {
        world.newsletters.remove(&email);
    }
    
    world.last_response = Some("success".to_string());
}

#[then(expr = "no emails with domain {string} should exist")]
async fn no_emails_with_domain_should_exist(world: &mut NewsletterWorld, domain: String) {
    let domain_emails = world
        .newsletters
        .values()
        .filter(|n| n.email.contains(&format!("@{}", domain)))
        .count();
    
    assert_eq!(domain_emails, 0, "No emails with domain {} should exist", domain);
}

// Email validation step using custom parameter type concept
#[then(expr = "the email {string} should be valid")]
async fn email_should_be_valid(_world: &mut NewsletterWorld, email: String) {
    // Simple email validation
    assert!(email.contains('@'), "Email {} should contain @ symbol", email);
    assert!(email.contains('.'), "Email {} should contain a dot", email);
    let parts: Vec<&str> = email.split('@').collect();
    assert_eq!(parts.len(), 2, "Email {} should have exactly one @ symbol", email);
    assert!(!parts[0].is_empty(), "Email {} should have a local part", email);
    assert!(!parts[1].is_empty(), "Email {} should have a domain part", email);
}

#[tokio::test]
async fn run_cucumber_expressions_tests() {
    NewsletterWorld::run("tests/features").await;
}