use async_trait::async_trait;
use tonic::{Request, Response, Status};

use crate::infrastructure::db::PgPool;
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
        let email = req.into_inner().email;

        // If you add repo::get_by_email later, switch to it for efficiency.
        let items = repo::list(&self.pool)
            .await
            .map_err(|e| Status::internal(format!("db error (list): {e}")))?;

        let active = items.iter().any(|n| n.email == email && n.active);

        Ok(Response::new(GetResponse { email, active }))
    }

    async fn subscribe(&self, req: Request<SubscribeRequest>) -> Result<Response<()>, Status> {
        let email = req.into_inner().email;
        repo::add(&self.pool, &email)
            .await
            .map_err(|e| Status::internal(format!("db error (add): {e}")))?;
        Ok(Response::new(()))
    }

    async fn un_subscribe(&self, req: Request<UnSubscribeRequest>) -> Result<Response<()>, Status> {
        let email = req.into_inner().email;
        repo::delete(&self.pool, &email)
            .await
            .map_err(|e| Status::internal(format!("db error (delete): {e}")))?;
        Ok(Response::new(()))
    }

    async fn list(&self, _req: Request<()>) -> Result<Response<ListResponse>, Status> {
        let items = repo::list(&self.pool)
            .await
            .map_err(|e| Status::internal(format!("db error (list): {e}")))?;

        let newsletters: Vec<Newsletter> = items.into_iter().map(Self::to_proto).collect();
        Ok(Response::new(ListResponse { newsletters }))
    }

    async fn update_status(
        &self,
        req: Request<UpdateStatusRequest>,
    ) -> Result<Response<()>, Status> {
        let UpdateStatusRequest { emails, active } = req.into_inner();

        if active {
            // re-subscribe all (ON CONFLICT DO NOTHING in repo::add)
            for email in emails {
                repo::add(&self.pool, &email)
                    .await
                    .map_err(|e| Status::internal(format!("db error (add): {e}")))?;
            }
        } else {
            // unsubscribe all
            for email in emails {
                repo::delete(&self.pool, &email)
                    .await
                    .map_err(|e| Status::internal(format!("db error (delete): {e}")))?;
            }
        }

        Ok(Response::new(()))
    }

    async fn delete(&self, req: Request<DeleteRequest>) -> Result<Response<()>, Status> {
        for email in req.into_inner().emails {
            repo::delete(&self.pool, &email)
                .await
                .map_err(|e| Status::internal(format!("db error (delete): {e}")))?;
        }
        Ok(Response::new(()))
    }
}
