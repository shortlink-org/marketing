use async_trait::async_trait;
use anyhow::Result;
use std::sync::Arc;

use crate::domain::newsletter::Newsletter;
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
}