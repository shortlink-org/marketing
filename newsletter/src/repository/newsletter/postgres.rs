use crate::domain::newsletter::newsletter::Newsletter;
use crate::infrastructure::db::db_schema::newsletters;
use crate::infrastructure::db::PgPool;

use anyhow::Result;
use diesel::prelude::*;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use tracing::{info, error, instrument};

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

#[instrument(skip(pool))]
pub async fn list(pool: &PgPool) -> Result<Vec<Newsletter>> {
    info!(entity = "newsletter_table", crud_operation = "READ", "Starting database list operation");

    let mut conn = match pool.get().await {
        Ok(conn) => {
            info!(entity = "newsletter_table", "Successfully acquired database connection");
            conn
        }
        Err(e) => {
            error!(entity = "newsletter_table", error = %e, "Failed to acquire database connection");
            return Err(e.into());
        }
    };

    let rows: Vec<NewsletterRow> = match newsletters::table
        .select(NewsletterRow::as_select())
        .order(newsletters::id.desc())
        .load(&mut conn)
        .await
    {
        Ok(rows) => {
            info!(entity = "newsletter_table", crud_operation = "READ", rows_count = rows.len(), "Successfully retrieved newsletters from database");
            rows
        }
        Err(e) => {
            error!(entity = "newsletter_table", crud_operation = "READ", error = %e, "Failed to retrieve newsletters from database");
            return Err(e.into());
        }
    };

    Ok(rows
        .into_iter()
        .map(|r| Newsletter {
            email: r.email,
            active: r.active,
        })
        .collect())
}

#[instrument(skip(pool), fields(email = %email))]
pub async fn add(pool: &PgPool, email: &str) -> Result<()> {
    info!(entity = "newsletter_table", crud_operation = "CREATE", email = %email, "Starting database add operation");

    let mut conn = match pool.get().await {
        Ok(conn) => {
            info!(entity = "newsletter_table", email = %email, "Successfully acquired database connection");
            conn
        }
        Err(e) => {
            error!(entity = "newsletter_table", crud_operation = "CREATE", email = %email, error = %e, "Failed to acquire database connection");
            return Err(e.into());
        }
    };

    match diesel::insert_into(newsletters::table)
        .values(&NewNewsletter {
            email,
            active: true,
        })
        .on_conflict(newsletters::email)
        .do_nothing()
        .execute(&mut conn)
        .await
    {
        Ok(_) => {
            info!(entity = "newsletter_table", crud_operation = "CREATE", email = %email, "Successfully added newsletter to database");
            Ok(())
        }
        Err(e) => {
            error!(entity = "newsletter_table", crud_operation = "CREATE", email = %email, error = %e, "Failed to add newsletter to database");
            Err(e.into())
        }
    }
}

#[instrument(skip(pool), fields(email = %email))]
pub async fn delete(pool: &PgPool, email: &str) -> Result<()> {
    info!(entity = "newsletter_table", crud_operation = "DELETE", email = %email, "Starting database delete operation");

    let mut conn = match pool.get().await {
        Ok(conn) => {
            info!(entity = "newsletter_table", email = %email, "Successfully acquired database connection");
            conn
        }
        Err(e) => {
            error!(entity = "newsletter_table", crud_operation = "DELETE", email = %email, error = %e, "Failed to acquire database connection");
            return Err(e.into());
        }
    };

    match diesel::delete(newsletters::table.filter(newsletters::email.eq(email)))
        .execute(&mut conn)
        .await
    {
        Ok(rows_affected) => {
            info!(entity = "newsletter_table", crud_operation = "DELETE", email = %email, rows_affected = rows_affected, "Successfully deleted newsletter from database");
            Ok(())
        }
        Err(e) => {
            error!(entity = "newsletter_table", crud_operation = "DELETE", email = %email, error = %e, "Failed to delete newsletter from database");
            Err(e.into())
        }
    }
}
