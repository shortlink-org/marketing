use async_trait::async_trait;
use tonic::{Request, Response, Status};
use tracing::{info, error, instrument, Span};

use crate::infrastructure::db::PgPool;
use crate::infrastructure::logging;
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
    #[instrument(skip(self), fields(email = %req.get_ref().email, trace_id))] 
    async fn get(&self, req: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        // Set trace_id from header or generate new one
        let trace_id = if let Some(trace_id) = logging::extract_trace_id_from_request(&req) {
            trace_id
        } else {
            uuid::Uuid::new_v4().to_string()
        };
        Span::current().record("trace_id", &trace_id);
        
        let email = req.into_inner().email;

        info!(operation = "get", crud_operation = "READ", entity = "newsletter", email = %email, "Starting get operation");

        let items = match repo::list(&self.pool).await {
            Ok(items) => {
                info!(operation = "get", crud_operation = "READ", entity = "newsletter", email = %email, count = items.len(), "Successfully retrieved newsletter list");
                items
            }
            Err(e) => {
                error!(operation = "get", crud_operation = "READ", entity = "newsletter", email = %email, error = %e, "Failed to retrieve newsletter list");
                return Err(Status::internal(format!("db error (list): {e}")));
            }
        };

        let active = items.iter().any(|n| n.email == email && n.active);
        info!(operation = "get", email = %email, active = active, "Get operation completed");

        Ok(Response::new(GetResponse { email, active }))
    }

    #[instrument(skip(self), fields(email = %req.get_ref().email, trace_id))]
    async fn subscribe(&self, req: Request<SubscribeRequest>) -> Result<Response<()>, Status> {
        // Set trace_id from header or generate new one
        let trace_id = if let Some(trace_id) = logging::extract_trace_id_from_request(&req) {
            trace_id
        } else {
            uuid::Uuid::new_v4().to_string()
        };
        Span::current().record("trace_id", &trace_id);
        
        let email = req.into_inner().email;

        info!(operation = "subscribe", crud_operation = "CREATE", entity = "newsletter", email = %email, "Starting subscribe operation");

        match repo::add(&self.pool, &email).await {
            Ok(_) => {
                info!(operation = "subscribe", crud_operation = "CREATE", entity = "newsletter", email = %email, "Successfully subscribed to newsletter");
                Ok(Response::new(()))
            }
            Err(e) => {
                error!(operation = "subscribe", crud_operation = "CREATE", entity = "newsletter", email = %email, error = %e, "Failed to subscribe to newsletter");
                Err(Status::internal(format!("db error (add): {e}")))
            }
        }
    }

    #[instrument(skip(self), fields(email = %req.get_ref().email, trace_id))]
    async fn un_subscribe(&self, req: Request<UnSubscribeRequest>) -> Result<Response<()>, Status> {
        // Set trace_id from header or generate new one
        let trace_id = if let Some(trace_id) = logging::extract_trace_id_from_request(&req) {
            trace_id
        } else {
            uuid::Uuid::new_v4().to_string()
        };
        Span::current().record("trace_id", &trace_id);
        
        let email = req.into_inner().email;

        info!(operation = "unsubscribe", crud_operation = "DELETE", entity = "newsletter", email = %email, "Starting unsubscribe operation");

        match repo::delete(&self.pool, &email).await {
            Ok(_) => {
                info!(operation = "unsubscribe", crud_operation = "DELETE", entity = "newsletter", email = %email, "Successfully unsubscribed from newsletter");
                Ok(Response::new(()))
            }
            Err(e) => {
                error!(operation = "unsubscribe", crud_operation = "DELETE", entity = "newsletter", email = %email, error = %e, "Failed to unsubscribe from newsletter");
                Err(Status::internal(format!("db error (delete): {e}")))
            }
        }
    }

    #[instrument(skip(self), fields(trace_id))]
    async fn list(&self, req: Request<()>) -> Result<Response<ListResponse>, Status> {
        // Set trace_id from header or generate new one
        let trace_id = if let Some(trace_id) = logging::extract_trace_id_from_request(&req) {
            trace_id
        } else {
            uuid::Uuid::new_v4().to_string()
        };
        Span::current().record("trace_id", &trace_id);

        info!(operation = "list", crud_operation = "READ", entity = "newsletter", "Starting list operation");

        let items = match repo::list(&self.pool).await {
            Ok(items) => {
                info!(operation = "list", crud_operation = "READ", entity = "newsletter", count = items.len(), "Successfully retrieved newsletter list");
                items
            }
            Err(e) => {
                error!(operation = "list", crud_operation = "READ", entity = "newsletter", error = %e, "Failed to retrieve newsletter list");
                return Err(Status::internal(format!("db error (list): {e}")));
            }
        };

        let newsletters: Vec<Newsletter> = items.into_iter().map(Self::to_proto).collect();
        Ok(Response::new(ListResponse { newsletters }))
    }

    #[instrument(skip(self), fields(emails = ?req.get_ref().emails, active = req.get_ref().active, trace_id))]
    async fn update_status(
        &self,
        req: Request<UpdateStatusRequest>,
    ) -> Result<Response<()>, Status> {
        // Set trace_id from header or generate new one
        let trace_id = if let Some(trace_id) = logging::extract_trace_id_from_request(&req) {
            trace_id
        } else {
            uuid::Uuid::new_v4().to_string()
        };
        Span::current().record("trace_id", &trace_id);
        
        let UpdateStatusRequest { emails, active } = req.into_inner();

        let operation = if active { "UPDATE_ACTIVATE" } else { "UPDATE_DEACTIVATE" };
        info!(operation = "update_status", crud_operation = operation, entity = "newsletter", count = emails.len(), active = active, "Starting bulk update status operation");

        if active {
            // re-subscribe all (ON CONFLICT DO NOTHING in repo::add)
            for email in &emails {
                if let Err(e) = repo::add(&self.pool, email).await {
                    error!(operation = "update_status", crud_operation = operation, entity = "newsletter", email = %email, error = %e, "Failed to activate subscription");
                    return Err(Status::internal(format!("db error (add): {e}")));
                }
            }
        } else {
            // unsubscribe all
            for email in &emails {
                if let Err(e) = repo::delete(&self.pool, email).await {
                    error!(operation = "update_status", crud_operation = operation, entity = "newsletter", email = %email, error = %e, "Failed to deactivate subscription");
                    return Err(Status::internal(format!("db error (delete): {e}")));
                }
            }
        }

        info!(operation = "update_status", crud_operation = operation, entity = "newsletter", count = emails.len(), active = active, "Successfully completed bulk update status operation");
        Ok(Response::new(()))
    }

    #[instrument(skip(self), fields(emails = ?req.get_ref().emails, trace_id))]
    async fn delete(&self, req: Request<DeleteRequest>) -> Result<Response<()>, Status> {
        // Set trace_id from header or generate new one
        let trace_id = if let Some(trace_id) = logging::extract_trace_id_from_request(&req) {
            trace_id
        } else {
            uuid::Uuid::new_v4().to_string()
        };
        Span::current().record("trace_id", &trace_id);
        
        let emails = req.into_inner().emails;

        info!(operation = "delete", crud_operation = "DELETE", entity = "newsletter", count = emails.len(), "Starting bulk delete operation");

        for email in &emails {
            if let Err(e) = repo::delete(&self.pool, email).await {
                error!(operation = "delete", crud_operation = "DELETE", entity = "newsletter", email = %email, error = %e, "Failed to delete subscription");
                return Err(Status::internal(format!("db error (delete): {e}")));
            }
        }

        info!(operation = "delete", crud_operation = "DELETE", entity = "newsletter", count = emails.len(), "Successfully completed bulk delete operation");
        Ok(Response::new(()))
    }
}
