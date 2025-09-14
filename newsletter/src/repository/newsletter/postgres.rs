use crate::domain::newsletter::newsletter::Newsletter;
use crate::infrastructure::db::db_schema::newsletters;
use crate::infrastructure::db::PgPool;
use crate::repository::newsletter::NewsletterRepository;

use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::SelectableHelper;
// <- for .as_select()
use diesel_async::RunQueryDsl;
// <- async query DSL

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = newsletters)]
#[diesel(check_for_backend(diesel::pg::Pg))] // optional: extra compile-time checks
struct NewsletterRow {
    #[allow(dead_code)]
    pub id: i64,
    pub email: String,
    pub active: bool,
    #[allow(dead_code)]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = newsletters)]
#[diesel(check_for_backend(diesel::pg::Pg))] // optional
struct NewNewsletter<'a> {
    pub email: &'a str,
    pub active: bool,
}

/// PostgreSQL implementation of the NewsletterRepository trait
#[derive(Clone)]
pub struct PostgresNewsletterRepository {
    pool: PgPool,
}

impl PostgresNewsletterRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NewsletterRepository for PostgresNewsletterRepository {
    async fn list(&self) -> Result<Vec<Newsletter>> {
        let mut conn = self.pool.get().await?;
        let rows: Vec<NewsletterRow> = newsletters::table
            .select(NewsletterRow::as_select())
            .order(newsletters::id.desc())
            .load(&mut conn)
            .await?;

        Ok(rows
            .into_iter()
            .map(|r| Newsletter {
                email: r.email,
                active: r.active,
            })
            .collect())
    }

    async fn add(&self, email: &str) -> Result<()> {
        let mut conn = self.pool.get().await?;
        diesel::insert_into(newsletters::table)
            .values(&NewNewsletter {
                email,
                active: true,
            })
            .on_conflict(newsletters::email)
            .do_nothing()
            .execute(&mut conn)
            .await?;
        Ok(())
    }

    async fn delete(&self, email: &str) -> Result<()> {
        let mut conn = self.pool.get().await?;
        diesel::delete(newsletters::table.filter(newsletters::email.eq(email)))
            .execute(&mut conn)
            .await?;
        Ok(())
    }

    async fn get_by_email(&self, email: &str) -> Result<Option<Newsletter>> {
        let mut conn = self.pool.get().await?;
        let row: Option<NewsletterRow> = newsletters::table
            .filter(newsletters::email.eq(email))
            .select(NewsletterRow::as_select())
            .first(&mut conn)
            .await
            .optional()?;

        Ok(row.map(|r| Newsletter {
            email: r.email,
            active: r.active,
        }))
    }
}

// Legacy functions - kept for backward compatibility if needed
// The trait-based implementation above should be used instead

#[allow(dead_code)]
pub async fn list(pool: &PgPool) -> Result<Vec<Newsletter>> {
    let repository = PostgresNewsletterRepository::new(pool.clone());
    repository.list().await
}

#[allow(dead_code)]
pub async fn add(pool: &PgPool, email: &str) -> Result<()> {
    let repository = PostgresNewsletterRepository::new(pool.clone());
    repository.add(email).await
}

#[allow(dead_code)]
pub async fn delete(pool: &PgPool, email: &str) -> Result<()> {
    let repository = PostgresNewsletterRepository::new(pool.clone());
    repository.delete(email).await
}
