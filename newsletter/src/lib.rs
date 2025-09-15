pub mod domain;
pub mod infrastructure;
pub mod repository;
pub mod service;

// Re-export commonly used items for easier testing access
#[cfg(test)]
pub use infrastructure::db::{build_pool_with_url, run_migrations_with_url, PgPool};