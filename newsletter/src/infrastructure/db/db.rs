//! Async Diesel pool + embedded migrations (bb8 via diesel_async).

use std::env;

use diesel::pg::PgConnection;
use diesel::Connection;
// brings `PgConnection::establish` into scope
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use diesel_async::{
    pooled_connection::{bb8::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};

/// Pool type (bb8 re-exported by `diesel_async`)
pub type PgPool = Pool<AsyncPgConnection>;

/// IMPORTANT: path is relative to this crate’s Cargo.toml.
/// Your migrations live under `src/infrastructure/db/migrations`, so use that:
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/infrastructure/db/migrations");

/// Build a pool for `AsyncPgConnection`.
pub async fn build_pool() -> anyhow::Result<PgPool> {
    let url = env::var("DATABASE_URL").map_err(|e| anyhow::anyhow!("DATABASE_URL not set: {e}"))?;
    build_pool_with_url(&url).await
}

/// Build a pool with a specific database URL (useful for testing).
pub async fn build_pool_with_url(url: &str) -> anyhow::Result<PgPool> {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);
    let pool = Pool::builder().max_size(16).build(manager).await?;
    Ok(pool)
}

/// Run embedded migrations on a blocking thread with a sync PgConnection.
pub async fn run_migrations() -> anyhow::Result<()> {
    let url = env::var("DATABASE_URL").map_err(|e| anyhow::anyhow!("DATABASE_URL not set: {e}"))?;
    run_migrations_with_url(&url).await
}

/// Run migrations with a specific database URL (useful for testing).
pub async fn run_migrations_with_url(url: &str) -> anyhow::Result<()> {
    let url = url.to_string();
    
    tokio::task::spawn_blocking(move || -> Result<(), anyhow::Error> {
        let mut conn = PgConnection::establish(&url).map_err(anyhow::Error::new)?;
        // This returns Result<_, Box<dyn Error + Send + Sync>> — map explicitly to anyhow
        conn.run_pending_migrations(MIGRATIONS)
            .map_err(|e| anyhow::anyhow!(e))?;
        Ok(())
    })
    .await??;

    Ok(())
}
