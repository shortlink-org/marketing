use async_trait::async_trait;
use anyhow::Result;
use crate::domain::newsletter::Newsletter;

pub mod postgres;

/// Repository trait for newsletter operations
#[async_trait]
pub trait NewsletterRepository: Send + Sync {
    /// Get all newsletters
    async fn list(&self) -> Result<Vec<Newsletter>>;
    
    /// Add a new newsletter subscription
    async fn add(&self, email: &str) -> Result<()>;
    
    /// Delete a newsletter subscription
    async fn delete(&self, email: &str) -> Result<()>;
    
    /// Get a newsletter by email (optional - for future use)
    async fn get_by_email(&self, email: &str) -> Result<Option<Newsletter>>;
}
