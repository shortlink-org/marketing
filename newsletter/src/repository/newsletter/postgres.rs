use crate::domain::newsletter::newsletter::Newsletter;
use crate::infrastructure::db::db_schema::newsletters;
use crate::infrastructure::db::PgPool;

use anyhow::Result;
use diesel::prelude::*;
use diesel::SelectableHelper;
// <- for .as_select()
use diesel_async::RunQueryDsl;
// <- async query DSL

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = newsletters)]
#[diesel(check_for_backend(diesel::pg::Pg))] // optional: extra compile-time checks
struct NewsletterRow {
    pub id: i64,
    pub email: String,
    pub active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = newsletters)]
#[diesel(check_for_backend(diesel::pg::Pg))] // optional
struct NewNewsletter<'a> {
    pub email: &'a str,
    pub active: bool,
}

pub async fn list(pool: &PgPool) -> Result<Vec<Newsletter>> {
    let mut conn = pool.get().await?; // bb8 RunError -> anyhow::Error
    let rows: Vec<NewsletterRow> = newsletters::table
        .select(NewsletterRow::as_select()) // requires SelectableHelper
        .order(newsletters::id.desc())
        .load(&mut conn)
        .await?; // Diesel error -> anyhow::Error

    Ok(rows
        .into_iter()
        .map(|r| Newsletter {
            email: r.email,
            active: r.active,
        })
        .collect())
}

pub async fn add(pool: &PgPool, email: &str) -> Result<()> {
    let mut conn = pool.get().await?;
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

pub async fn delete(pool: &PgPool, email: &str) -> Result<()> {
    let mut conn = pool.get().await?;
    diesel::delete(newsletters::table.filter(newsletters::email.eq(email)))
        .execute(&mut conn)
        .await?;
    Ok(())
}
