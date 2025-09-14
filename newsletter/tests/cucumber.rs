use cucumber::{given, then, when, World};
use std::collections::HashMap;
use std::env;

use newsletter::infrastructure::db::{build_pool_with_url, run_migrations_with_url, PgPool};
use newsletter::infrastructure::rpc::newsletter::v1::proto::{
    ListResponse, Newsletter,
};

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct NewsletterWorld {
    pub pool: Option<PgPool>,
    pub last_response: Option<String>,
    pub subscriptions: HashMap<String, bool>,
    pub last_list_response: Option<ListResponse>,
    pub last_get_response: Option<(String, bool)>,
}

impl Default for NewsletterWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl NewsletterWorld {
    pub fn new() -> Self {
        Self {
            pool: None,
            last_response: None,
            subscriptions: HashMap::new(),
            last_list_response: None,
            last_get_response: None,
        }
    }

    pub async fn setup_database(&mut self) -> anyhow::Result<()> {
        // Use environment variable for test database URL
        // In a real scenario, you'd set this to a test database
        let connection_string = env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:password@localhost/newsletter_test".to_string());

        // Setup database pool and run migrations
        let pool = build_pool_with_url(&connection_string).await?;
        run_migrations_with_url(&connection_string).await?;

        self.pool = Some(pool);
        Ok(())
    }

    pub async fn cleanup_database(&mut self) -> anyhow::Result<()> {
        if let Some(pool) = &self.pool {
            use newsletter::infrastructure::db::db_schema::newsletters;
            use diesel::prelude::*;
            use diesel_async::RunQueryDsl;

            let mut conn = pool.get().await?;
            diesel::delete(newsletters::table)
                .execute(&mut conn)
                .await?;
        }
        Ok(())
    }
}

// Background steps
#[given("the newsletter service is running")]
async fn service_is_running(world: &mut NewsletterWorld) {
    // For now, we'll skip the database setup in tests since we don't have a test DB
    // In a real scenario, you'd setup a test database here
    world.last_response = Some("service_running".to_string());
}

#[given("the database is clean")]
async fn database_is_clean(world: &mut NewsletterWorld) {
    world.cleanup_database().await.expect("Failed to cleanup database");
}

// Create operations
#[when(regex = r"^I subscribe email \"([^\"]+)\"$")]
async fn subscribe_email(world: &mut NewsletterWorld, email: String) {
    let pool = world.pool.as_ref().expect("Database pool not initialized");
    
    use newsletter::repository::newsletter::postgres as repo;
    let result = repo::add(pool, &email).await;
    
    match result {
        Ok(_) => {
            world.subscriptions.insert(email, true);
            world.last_response = Some("success".to_string());
        }
        Err(e) => {
            world.last_response = Some(format!("error: {}", e));
        }
    }
}

#[given(regex = r"^I have subscribed email \"([^\"]+)\"$")]
async fn have_subscribed_email(world: &mut NewsletterWorld, email: String) {
    subscribe_email(world, email).await;
}

#[when(regex = r"^I subscribe email \"([^\"]+)\" again$")]
async fn subscribe_email_again(world: &mut NewsletterWorld, email: String) {
    subscribe_email(world, email).await;
}

#[given("I have subscribed the following emails:")]
async fn have_subscribed_emails(world: &mut NewsletterWorld, table: cucumber::gherkin::Table) {
    for row in table.rows.iter().skip(1) { // Skip header
        if let Some(email) = row.get(0) {
            subscribe_email(world, email.clone()).await;
        }
    }
}

// Read operations
#[when(regex = r"^I get the subscription for \"([^\"]+)\"$")]
async fn get_subscription(world: &mut NewsletterWorld, email: String) {
    let pool = world.pool.as_ref().expect("Database pool not initialized");
    
    use newsletter::repository::newsletter::postgres as repo;
    match repo::list(pool).await {
        Ok(newsletters) => {
            let found = newsletters.iter().find(|n| n.email == email);
            match found {
                Some(newsletter) => {
                    world.last_get_response = Some((newsletter.email.clone(), newsletter.active));
                }
                None => {
                    world.last_get_response = Some((email, false));
                }
            }
        }
        Err(e) => {
            world.last_response = Some(format!("error: {}", e));
        }
    }
}

#[when("I list all subscriptions")]
async fn list_all_subscriptions(world: &mut NewsletterWorld) {
    let pool = world.pool.as_ref().expect("Database pool not initialized");
    
    use newsletter::repository::newsletter::postgres as repo;
    match repo::list(pool).await {
        Ok(newsletters) => {
            let proto_newsletters: Vec<Newsletter> = newsletters
                .into_iter()
                .map(|n| Newsletter {
                    field_mask: None,
                    email: n.email,
                    active: n.active,
                })
                .collect();
            
            world.last_list_response = Some(ListResponse {
                newsletters: proto_newsletters,
            });
        }
        Err(e) => {
            world.last_response = Some(format!("error: {}", e));
        }
    }
}

// Update operations
#[when("I update status to inactive for emails:")]
async fn update_status_inactive(world: &mut NewsletterWorld, table: cucumber::gherkin::Table) {
    let pool = world.pool.as_ref().expect("Database pool not initialized");
    
    use newsletter::repository::newsletter::postgres as repo;
    for row in table.rows.iter().skip(1) { // Skip header
        if let Some(email) = row.get(0) {
            match repo::delete(pool, email).await {
                Ok(_) => {
                    world.subscriptions.insert(email.clone(), false);
                }
                Err(e) => {
                    world.last_response = Some(format!("error: {}", e));
                    return;
                }
            }
        }
    }
    world.last_response = Some("success".to_string());
}

#[when("I update status to active for emails:")]
async fn update_status_active(world: &mut NewsletterWorld, table: cucumber::gherkin::Table) {
    let pool = world.pool.as_ref().expect("Database pool not initialized");
    
    use newsletter::repository::newsletter::postgres as repo;
    for row in table.rows.iter().skip(1) { // Skip header
        if let Some(email) = row.get(0) {
            match repo::add(pool, email).await {
                Ok(_) => {
                    world.subscriptions.insert(email.clone(), true);
                }
                Err(e) => {
                    world.last_response = Some(format!("error: {}", e));
                    return;
                }
            }
        }
    }
    world.last_response = Some("success".to_string());
}

#[given("I have the following inactive emails in the database:")]
async fn have_inactive_emails(world: &mut NewsletterWorld, table: cucumber::gherkin::Table) {
    // For this test, we'll add and then remove emails to simulate inactive state
    for row in table.rows.iter().skip(1) { // Skip header
        if let Some(email) = row.get(0) {
            subscribe_email(world, email.clone()).await;
            update_status_inactive(world, cucumber::gherkin::Table {
                rows: vec![vec!["email".to_string()], vec![email.clone()]],
            }).await;
        }
    }
}

// Delete operations
#[when(regex = r"^I unsubscribe email \"([^\"]+)\"$")]
async fn unsubscribe_email(world: &mut NewsletterWorld, email: String) {
    let pool = world.pool.as_ref().expect("Database pool not initialized");
    
    use newsletter::repository::newsletter::postgres as repo;
    match repo::delete(pool, &email).await {
        Ok(_) => {
            world.subscriptions.remove(&email);
            world.last_response = Some("success".to_string());
        }
        Err(e) => {
            world.last_response = Some(format!("error: {}", e));
        }
    }
}

#[when("I delete the following emails:")]
async fn delete_emails(world: &mut NewsletterWorld, table: cucumber::gherkin::Table) {
    let pool = world.pool.as_ref().expect("Database pool not initialized");
    
    use newsletter::repository::newsletter::postgres as repo;
    for row in table.rows.iter().skip(1) { // Skip header
        if let Some(email) = row.get(0) {
            match repo::delete(pool, email).await {
                Ok(_) => {
                    world.subscriptions.remove(email);
                }
                Err(e) => {
                    world.last_response = Some(format!("error: {}", e));
                    return;
                }
            }
        }
    }
    world.last_response = Some("success".to_string());
}

// Assertion steps
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

#[then("the emails should be unsubscribed")]
async fn emails_unsubscribed(world: &mut NewsletterWorld) {
    assert_eq!(world.last_response, Some("success".to_string()));
}

#[then("the emails should be resubscribed")]
async fn emails_resubscribed(world: &mut NewsletterWorld) {
    assert_eq!(world.last_response, Some("success".to_string()));
}

#[then(regex = r"^the email \"([^\"]+)\" should be active$")]
async fn email_should_be_active(world: &mut NewsletterWorld, email: String) {
    let pool = world.pool.as_ref().expect("Database pool not initialized");
    
    use newsletter::repository::newsletter::postgres as repo;
    let newsletters = repo::list(pool).await.expect("Failed to list newsletters");
    let found = newsletters.iter().find(|n| n.email == email);
    
    assert!(found.is_some(), "Email {} should exist", email);
    assert!(found.unwrap().active, "Email {} should be active", email);
}

#[then(regex = r"^\"([^\"]+)\" should be active$")]
async fn email_active(world: &mut NewsletterWorld, email: String) {
    email_should_be_active(world, email).await;
}

#[then(regex = r"^\"([^\"]+)\" should not be active$")]
async fn email_should_not_be_active(world: &mut NewsletterWorld, email: String) {
    let pool = world.pool.as_ref().expect("Database pool not initialized");
    
    use newsletter::repository::newsletter::postgres as repo;
    let newsletters = repo::list(pool).await.expect("Failed to list newsletters");
    let found = newsletters.iter().find(|n| n.email == email);
    
    // Email should either not exist or be inactive
    assert!(found.is_none(), "Email {} should not exist (should be deleted)", email);
}

#[then(regex = r"^the email \"([^\"]+)\" should not exist$")]
async fn email_should_not_exist(world: &mut NewsletterWorld, email: String) {
    let pool = world.pool.as_ref().expect("Database pool not initialized");
    
    use newsletter::repository::newsletter::postgres as repo;
    let newsletters = repo::list(pool).await.expect("Failed to list newsletters");
    let found = newsletters.iter().find(|n| n.email == email);
    
    assert!(found.is_none(), "Email {} should not exist", email);
}

#[then(regex = r"^the email \"([^\"]+)\" should still exist$")]
async fn email_should_still_exist(world: &mut NewsletterWorld, email: String) {
    let pool = world.pool.as_ref().expect("Database pool not initialized");
    
    use newsletter::repository::newsletter::postgres as repo;
    let newsletters = repo::list(pool).await.expect("Failed to list newsletters");
    let found = newsletters.iter().find(|n| n.email == email);
    
    assert!(found.is_some(), "Email {} should still exist", email);
}

#[then(regex = r"^there should be only one subscription for \"([^\"]+)\"$")]
async fn only_one_subscription(world: &mut NewsletterWorld, email: String) {
    let pool = world.pool.as_ref().expect("Database pool not initialized");
    
    use newsletter::repository::newsletter::postgres as repo;
    let newsletters = repo::list(pool).await.expect("Failed to list newsletters");
    let count = newsletters.iter().filter(|n| n.email == email).count();
    
    assert_eq!(count, 1, "Should have exactly one subscription for {}", email);
}

#[then("the subscription should exist")]
async fn subscription_should_exist(world: &mut NewsletterWorld) {
    assert!(world.last_get_response.is_some(), "Should have a get response");
    let (_, active) = world.last_get_response.as_ref().unwrap();
    assert!(*active, "Subscription should exist and be active");
}

#[then("the subscription should not exist")]
async fn subscription_should_not_exist(world: &mut NewsletterWorld) {
    assert!(world.last_get_response.is_some(), "Should have a get response");
    let (_, active) = world.last_get_response.as_ref().unwrap();
    assert!(!*active, "Subscription should not exist or be inactive");
}

#[then(regex = r"^the email should be \"([^\"]+)\"$")]
async fn email_should_be(world: &mut NewsletterWorld, expected_email: String) {
    assert!(world.last_get_response.is_some(), "Should have a get response");
    let (email, _) = world.last_get_response.as_ref().unwrap();
    assert_eq!(*email, expected_email, "Email should match expected value");
}

#[then("the status should be active")]
async fn status_should_be_active(world: &mut NewsletterWorld) {
    assert!(world.last_get_response.is_some(), "Should have a get response");
    let (_, active) = world.last_get_response.as_ref().unwrap();
    assert!(*active, "Status should be active");
}

#[then("the status should be inactive")]
async fn status_should_be_inactive(world: &mut NewsletterWorld) {
    assert!(world.last_get_response.is_some(), "Should have a get response");
    let (_, active) = world.last_get_response.as_ref().unwrap();
    assert!(!*active, "Status should be inactive");
}

#[then(regex = r"^I should get (\d+) subscriptions$")]
async fn should_get_subscriptions_count(world: &mut NewsletterWorld, count: usize) {
    assert!(world.last_list_response.is_some(), "Should have a list response");
    let response = world.last_list_response.as_ref().unwrap();
    assert_eq!(response.newsletters.len(), count, "Should have {} subscriptions", count);
}

#[then(regex = r"^the list should contain \"([^\"]+)\"$")]
async fn list_should_contain_email(world: &mut NewsletterWorld, email: String) {
    assert!(world.last_list_response.is_some(), "Should have a list response");
    let response = world.last_list_response.as_ref().unwrap();
    let found = response.newsletters.iter().any(|n| n.email == email);
    assert!(found, "List should contain email {}", email);
}

#[tokio::main]
async fn main() {
    NewsletterWorld::run("tests/features").await;
}