// This example demonstrates how to use trait-based dependency injection
// for the newsletter service

use std::sync::Arc;
use newsletter::{
    repository::newsletter::{NewsletterRepository, postgres::PostgresNewsletterRepository},
    service::newsletter::{NewsletterService, DefaultNewsletterService},
    infrastructure::db::{build_pool, run_migrations},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the database connection pool
    let pool = build_pool().await?;
    run_migrations().await?;
    
    // Create the repository with dependency injection
    let repository: Arc<dyn NewsletterRepository> = Arc::new(
        PostgresNewsletterRepository::new(pool)
    );
    
    // Create the service with dependency injection
    let service: Arc<dyn NewsletterService> = Arc::new(
        DefaultNewsletterService::new(repository)
    );
    
    // Example usage of the service
    println!("Trait-based Dependency Injection Demo");
    println!("=====================================");
    
    // Subscribe a test email
    println!("Subscribing test@example.com...");
    service.subscribe("test@example.com").await?;
    
    // Check subscription status
    let is_subscribed = service.get_subscription_status("test@example.com").await?;
    println!("Is test@example.com subscribed? {}", is_subscribed);
    
    // List all newsletters
    let newsletters = service.list_newsletters().await?;
    println!("Total newsletters: {}", newsletters.len());
    
    // Unsubscribe
    println!("Unsubscribing test@example.com...");
    service.unsubscribe("test@example.com").await?;
    
    // Check subscription status again
    let is_subscribed = service.get_subscription_status("test@example.com").await?;
    println!("Is test@example.com subscribed after unsubscribe? {}", is_subscribed);
    
    println!("Demo completed successfully!");
    Ok(())
}