use async_trait::async_trait;
use tonic::{Request, Response, Status};
use serde_json::json;
use std::collections::HashMap;

use crate::infrastructure::db::PgPool;
use crate::infrastructure::logging::{self, log_crud_start, log_crud_success, log_crud_error};
use crate::repository::newsletter::postgres as repo;

use crate::infrastructure::rpc::newsletter::v1::proto::{
    newsletter_service_server::NewsletterService, DeleteRequest, GetRequest, GetResponse,
    ListResponse, Newsletter, SubscribeRequest, UnSubscribeRequest, UpdateStatusRequest,
};

#[derive(Clone)]
pub struct MyNewsletterService {
    pool: PgPool,
}

impl MyNewsletterService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn to_proto(n: crate::domain::newsletter::newsletter::Newsletter) -> Newsletter {
        Newsletter {
            field_mask: None,
            email: n.email,
            active: n.active,
        }
    }
}

#[async_trait]
impl NewsletterService for MyNewsletterService {
    async fn get(&self, req: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let trace_ctx = logging::get_or_create_trace_context(&req, "newsletter.get");
        let email = req.into_inner().email;

        let mut details = HashMap::new();
        details.insert("email".to_string(), json!(email));
        log_crud_start(&trace_ctx, "READ", "newsletter", Some(details.clone()));

        // If you add repo::get_by_email later, switch to it for efficiency.
        let items = match repo::list(&self.pool).await {
            Ok(items) => {
                log_crud_success(&trace_ctx, "READ", "newsletter", Some(details.clone()));
                items
            }
            Err(e) => {
                log_crud_error(&trace_ctx, "READ", "newsletter", &e.to_string(), Some(details));
                return Err(Status::internal(format!("db error (list): {e}")));
            }
        };

        let active = items.iter().any(|n| n.email == email && n.active);

        Ok(Response::new(GetResponse { email, active }))
    }

    async fn subscribe(&self, req: Request<SubscribeRequest>) -> Result<Response<()>, Status> {
        let trace_ctx = logging::get_or_create_trace_context(&req, "newsletter.subscribe");
        let email = req.into_inner().email;

        let mut details = HashMap::new();
        details.insert("email".to_string(), json!(email));
        log_crud_start(&trace_ctx, "CREATE", "newsletter", Some(details.clone()));

        match repo::add(&self.pool, &email).await {
            Ok(_) => {
                log_crud_success(&trace_ctx, "CREATE", "newsletter", Some(details));
                Ok(Response::new(()))
            }
            Err(e) => {
                log_crud_error(&trace_ctx, "CREATE", "newsletter", &e.to_string(), Some(details));
                Err(Status::internal(format!("db error (add): {e}")))
            }
        }
    }

    async fn un_subscribe(&self, req: Request<UnSubscribeRequest>) -> Result<Response<()>, Status> {
        let trace_ctx = logging::get_or_create_trace_context(&req, "newsletter.unsubscribe");
        let email = req.into_inner().email;

        let mut details = HashMap::new();
        details.insert("email".to_string(), json!(email));
        log_crud_start(&trace_ctx, "DELETE", "newsletter", Some(details.clone()));

        match repo::delete(&self.pool, &email).await {
            Ok(_) => {
                log_crud_success(&trace_ctx, "DELETE", "newsletter", Some(details));
                Ok(Response::new(()))
            }
            Err(e) => {
                log_crud_error(&trace_ctx, "DELETE", "newsletter", &e.to_string(), Some(details));
                Err(Status::internal(format!("db error (delete): {e}")))
            }
        }
    }

    async fn list(&self, req: Request<()>) -> Result<Response<ListResponse>, Status> {
        let trace_ctx = logging::get_or_create_trace_context(&req, "newsletter.list");
        
        log_crud_start(&trace_ctx, "READ", "newsletter", None);

        let items = match repo::list(&self.pool).await {
            Ok(items) => {
                let mut details = HashMap::new();
                details.insert("count".to_string(), json!(items.len()));
                log_crud_success(&trace_ctx, "READ", "newsletter", Some(details));
                items
            }
            Err(e) => {
                log_crud_error(&trace_ctx, "READ", "newsletter", &e.to_string(), None);
                return Err(Status::internal(format!("db error (list): {e}")));
            }
        };

        let newsletters: Vec<Newsletter> = items.into_iter().map(Self::to_proto).collect();
        Ok(Response::new(ListResponse { newsletters }))
    }

    async fn update_status(
        &self,
        req: Request<UpdateStatusRequest>,
    ) -> Result<Response<()>, Status> {
        let trace_ctx = logging::get_or_create_trace_context(&req, "newsletter.update_status");
        let UpdateStatusRequest { emails, active } = req.into_inner();

        let mut details = HashMap::new();
        details.insert("emails".to_string(), json!(emails));
        details.insert("active".to_string(), json!(active));
        details.insert("count".to_string(), json!(emails.len()));
        
        let operation = if active { "UPDATE_ACTIVATE" } else { "UPDATE_DEACTIVATE" };
        log_crud_start(&trace_ctx, operation, "newsletter", Some(details.clone()));

        if active {
            // re-subscribe all (ON CONFLICT DO NOTHING in repo::add)
            for email in emails {
                if let Err(e) = repo::add(&self.pool, &email).await {
                    log_crud_error(&trace_ctx, operation, "newsletter", &e.to_string(), Some(details));
                    return Err(Status::internal(format!("db error (add): {e}")));
                }
            }
        } else {
            // unsubscribe all
            for email in emails {
                if let Err(e) = repo::delete(&self.pool, &email).await {
                    log_crud_error(&trace_ctx, operation, "newsletter", &e.to_string(), Some(details));
                    return Err(Status::internal(format!("db error (delete): {e}")));
                }
            }
        }

        log_crud_success(&trace_ctx, operation, "newsletter", Some(details));
        Ok(Response::new(()))
    }

    async fn delete(&self, req: Request<DeleteRequest>) -> Result<Response<()>, Status> {
        let trace_ctx = logging::get_or_create_trace_context(&req, "newsletter.delete");
        let emails = req.into_inner().emails;

        let mut details = HashMap::new();
        details.insert("emails".to_string(), json!(emails));
        details.insert("count".to_string(), json!(emails.len()));
        log_crud_start(&trace_ctx, "DELETE", "newsletter", Some(details.clone()));

        for email in emails {
            if let Err(e) = repo::delete(&self.pool, &email).await {
                log_crud_error(&trace_ctx, "DELETE", "newsletter", &e.to_string(), Some(details));
                return Err(Status::internal(format!("db error (delete): {e}")));
            }
        }

        log_crud_success(&trace_ctx, "DELETE", "newsletter", Some(details));
        Ok(Response::new(()))
    }
}
