use crate::domain::newsletter::newsletter::Newsletter;
use crate::infrastructure::db::db_schema::newsletters;
use crate::infrastructure::db::PgPool;
use crate::infrastructure::logging::{TraceContext, log_crud_start, log_crud_success, log_crud_error};

use anyhow::Result;
use diesel::prelude::*;
use diesel::SelectableHelper;
// <- for .as_select()
use diesel_async::RunQueryDsl;
use serde_json::json;
use std::collections::HashMap;
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
    list_with_trace(pool, None).await
}

pub async fn list_with_trace(pool: &PgPool, trace_ctx: Option<&TraceContext>) -> Result<Vec<Newsletter>> {
    if let Some(ctx) = trace_ctx {
        log_crud_start(ctx, "READ", "newsletter_table", None);
    }

    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            if let Some(ctx) = trace_ctx {
                log_crud_error(ctx, "READ", "newsletter_table", &e.to_string(), None);
            }
            return Err(e.into());
        }
    };

    let rows: Vec<NewsletterRow> = match newsletters::table
        .select(NewsletterRow::as_select()) // requires SelectableHelper
        .order(newsletters::id.desc())
        .load(&mut conn)
        .await
    {
        Ok(rows) => {
            if let Some(ctx) = trace_ctx {
                let mut details = HashMap::new();
                details.insert("rows_count".to_string(), json!(rows.len()));
                log_crud_success(ctx, "READ", "newsletter_table", Some(details));
            }
            rows
        }
        Err(e) => {
            if let Some(ctx) = trace_ctx {
                log_crud_error(ctx, "READ", "newsletter_table", &e.to_string(), None);
            }
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

pub async fn add(pool: &PgPool, email: &str) -> Result<()> {
    add_with_trace(pool, email, None).await
}

pub async fn add_with_trace(pool: &PgPool, email: &str, trace_ctx: Option<&TraceContext>) -> Result<()> {
    if let Some(ctx) = trace_ctx {
        let mut details = HashMap::new();
        details.insert("email".to_string(), json!(email));
        log_crud_start(ctx, "CREATE", "newsletter_table", Some(details));
    }

    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            if let Some(ctx) = trace_ctx {
                let mut details = HashMap::new();
                details.insert("email".to_string(), json!(email));
                log_crud_error(ctx, "CREATE", "newsletter_table", &e.to_string(), Some(details));
            }
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
            if let Some(ctx) = trace_ctx {
                let mut details = HashMap::new();
                details.insert("email".to_string(), json!(email));
                log_crud_success(ctx, "CREATE", "newsletter_table", Some(details));
            }
            Ok(())
        }
        Err(e) => {
            if let Some(ctx) = trace_ctx {
                let mut details = HashMap::new();
                details.insert("email".to_string(), json!(email));
                log_crud_error(ctx, "CREATE", "newsletter_table", &e.to_string(), Some(details));
            }
            Err(e.into())
        }
    }
}

pub async fn delete(pool: &PgPool, email: &str) -> Result<()> {
    delete_with_trace(pool, email, None).await
}

pub async fn delete_with_trace(pool: &PgPool, email: &str, trace_ctx: Option<&TraceContext>) -> Result<()> {
    if let Some(ctx) = trace_ctx {
        let mut details = HashMap::new();
        details.insert("email".to_string(), json!(email));
        log_crud_start(ctx, "DELETE", "newsletter_table", Some(details));
    }

    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            if let Some(ctx) = trace_ctx {
                let mut details = HashMap::new();
                details.insert("email".to_string(), json!(email));
                log_crud_error(ctx, "DELETE", "newsletter_table", &e.to_string(), Some(details));
            }
            return Err(e.into());
        }
    };

    match diesel::delete(newsletters::table.filter(newsletters::email.eq(email)))
        .execute(&mut conn)
        .await
    {
        Ok(rows_affected) => {
            if let Some(ctx) = trace_ctx {
                let mut details = HashMap::new();
                details.insert("email".to_string(), json!(email));
                details.insert("rows_affected".to_string(), json!(rows_affected));
                log_crud_success(ctx, "DELETE", "newsletter_table", Some(details));
            }
            Ok(())
        }
        Err(e) => {
            if let Some(ctx) = trace_ctx {
                let mut details = HashMap::new();
                details.insert("email".to_string(), json!(email));
                log_crud_error(ctx, "DELETE", "newsletter_table", &e.to_string(), Some(details));
            }
            Err(e.into())
        }
    }
}
