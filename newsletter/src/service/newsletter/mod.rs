use async_trait::async_trait;
use anyhow::Result;
use std::sync::Arc;

use crate::domain::newsletter::newsletter::Newsletter;
use crate::repository::newsletter::NewsletterRepository;

/// Service trait for newsletter business logic operations
#[async_trait]
pub trait NewsletterService: Send + Sync {
    /// Get all newsletters
    async fn list_newsletters(&self) -> Result<Vec<Newsletter>>;
    
    /// Subscribe to newsletter
    async fn subscribe(&self, email: &str) -> Result<()>;
    
    /// Unsubscribe from newsletter
    async fn unsubscribe(&self, email: &str) -> Result<()>;
    
    /// Get newsletter subscription status by email
    async fn get_subscription_status(&self, email: &str) -> Result<bool>;
    
    /// Update subscription status for multiple emails
    async fn update_subscription_status(&self, emails: Vec<String>, active: bool) -> Result<()>;
    
    /// Delete multiple newsletter subscriptions
    async fn delete_subscriptions(&self, emails: Vec<String>) -> Result<()>;
    
    /// Demonstrate the trait-based dependency injection functionality
    /// This method showcases how the service layer uses the injected repository
    async fn demo_functionality(&self, test_email: &str) -> Result<String>;
}

/// Default implementation of the newsletter service
#[derive(Clone)]
pub struct DefaultNewsletterService<R: NewsletterRepository> {
    repository: Arc<R>,
}

impl<R: NewsletterRepository> DefaultNewsletterService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: NewsletterRepository + 'static> NewsletterService for DefaultNewsletterService<R> {
    async fn list_newsletters(&self) -> Result<Vec<Newsletter>> {
        self.repository.list().await
    }
    
    async fn subscribe(&self, email: &str) -> Result<()> {
        // Add business logic validation if needed
        if email.trim().is_empty() {
            return Err(anyhow::anyhow!("Email cannot be empty"));
        }
        
        // Basic email validation
        if !email.contains('@') {
            return Err(anyhow::anyhow!("Invalid email format"));
        }
        
        self.repository.add(email).await
    }
    
    async fn unsubscribe(&self, email: &str) -> Result<()> {
        if email.trim().is_empty() {
            return Err(anyhow::anyhow!("Email cannot be empty"));
        }
        
        self.repository.delete(email).await
    }
    
    async fn get_subscription_status(&self, email: &str) -> Result<bool> {
        match self.repository.get_by_email(email).await? {
            Some(newsletter) => Ok(newsletter.active),
            None => Ok(false),
        }
    }
    
    async fn update_subscription_status(&self, emails: Vec<String>, active: bool) -> Result<()> {
        for email in emails {
            if active {
                // Subscribe if not already subscribed
                self.repository.add(&email).await?;
            } else {
                // Unsubscribe
                self.repository.delete(&email).await?;
            }
        }
        Ok(())
    }
    
    async fn delete_subscriptions(&self, emails: Vec<String>) -> Result<()> {
        for email in emails {
            self.repository.delete(&email).await?;
        }
        Ok(())
    }
    
    async fn demo_functionality(&self, test_email: &str) -> Result<String> {
        let mut demo_log = String::new();
        demo_log.push_str("=== Trait-Based Dependency Injection Demo ===\n");
        
        // Step 1: Subscribe a test email
        demo_log.push_str(&format!("1. Subscribing {}...\n", test_email));
        match self.subscribe(test_email).await {
            Ok(_) => demo_log.push_str("   ✓ Successfully subscribed\n"),
            Err(e) => demo_log.push_str(&format!("   ✗ Failed to subscribe: {}\n", e)),
        }
        
        // Step 2: Check subscription status
        demo_log.push_str("2. Checking subscription status...\n");
        match self.get_subscription_status(test_email).await {
            Ok(status) => demo_log.push_str(&format!("   ✓ Subscription status: {}\n", status)),
            Err(e) => demo_log.push_str(&format!("   ✗ Failed to check status: {}\n", e)),
        }
        
        // Step 3: List all newsletters
        demo_log.push_str("3. Listing all newsletters...\n");
        match self.list_newsletters().await {
            Ok(newsletters) => {
                demo_log.push_str(&format!("   ✓ Found {} newsletters\n", newsletters.len()));
                for newsletter in newsletters.iter().take(3) {
                    demo_log.push_str(&format!("   - {}: {}\n", newsletter.email, 
                        if newsletter.active { "active" } else { "inactive" }));
                }
                if newsletters.len() > 3 {
                    demo_log.push_str(&format!("   ... and {} more\n", newsletters.len() - 3));
                }
            },
            Err(e) => demo_log.push_str(&format!("   ✗ Failed to list newsletters: {}\n", e)),
        }
        
        // Step 4: Unsubscribe
        demo_log.push_str("4. Unsubscribing test email...\n");
        match self.unsubscribe(test_email).await {
            Ok(_) => demo_log.push_str("   ✓ Successfully unsubscribed\n"),
            Err(e) => demo_log.push_str(&format!("   ✗ Failed to unsubscribe: {}\n", e)),
        }
        
        // Step 5: Verify unsubscription
        demo_log.push_str("5. Verifying unsubscription...\n");
        match self.get_subscription_status(test_email).await {
            Ok(status) => demo_log.push_str(&format!("   ✓ Final status: {}\n", status)),
            Err(e) => demo_log.push_str(&format!("   ✗ Failed to verify: {}\n", e)),
        }
        
        demo_log.push_str("=== Demo Complete ===\n");
        demo_log.push_str("This demo showcases:\n");
        demo_log.push_str("- Service layer using injected repository trait\n");
        demo_log.push_str("- Business logic validation and error handling\n");
        demo_log.push_str("- Clean separation of concerns\n");
        demo_log.push_str("- Type-safe dependency injection\n");
        
        Ok(demo_log)
    }
}