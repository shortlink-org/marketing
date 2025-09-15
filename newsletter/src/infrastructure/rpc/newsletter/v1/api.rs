use async_trait::async_trait;
use tonic::{Request, Response, Status};
use std::sync::Arc;

use crate::service::newsletter::NewsletterService as NewsletterServiceTrait;

use crate::infrastructure::rpc::newsletter::v1::proto::{
    newsletter_service_server::NewsletterService, DeleteRequest, DemoRequest, DemoResponse, 
    GetRequest, GetResponse, ListResponse, Newsletter, SubscribeRequest, UnSubscribeRequest, 
    UpdateStatusRequest,
};

#[derive(Clone)]
pub struct MyNewsletterService<S: NewsletterServiceTrait> {
    service: Arc<S>,
}

impl<S: NewsletterServiceTrait> MyNewsletterService<S> {
    pub fn new(service: Arc<S>) -> Self {
        Self { service }
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
impl<S: NewsletterServiceTrait + 'static> NewsletterService for MyNewsletterService<S> {
    async fn get(&self, req: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let email = req.into_inner().email;

        let active = self.service
            .get_subscription_status(&email)
            .await
            .map_err(|e| Status::internal(format!("service error (get_subscription_status): {e}")))?;

        Ok(Response::new(GetResponse { email, active }))
    }

    async fn subscribe(&self, req: Request<SubscribeRequest>) -> Result<Response<()>, Status> {
        let email = req.into_inner().email;
        self.service
            .subscribe(&email)
            .await
            .map_err(|e| Status::internal(format!("service error (subscribe): {e}")))?;
        Ok(Response::new(()))
    }

    async fn un_subscribe(&self, req: Request<UnSubscribeRequest>) -> Result<Response<()>, Status> {
        let email = req.into_inner().email;
        self.service
            .unsubscribe(&email)
            .await
            .map_err(|e| Status::internal(format!("service error (unsubscribe): {e}")))?;
        Ok(Response::new(()))
    }

    async fn list(&self, _req: Request<()>) -> Result<Response<ListResponse>, Status> {
        let items = self.service
            .list_newsletters()
            .await
            .map_err(|e| Status::internal(format!("service error (list_newsletters): {e}")))?;

        let newsletters: Vec<Newsletter> = items.into_iter().map(Self::to_proto).collect();
        Ok(Response::new(ListResponse { newsletters }))
    }

    async fn update_status(
        &self,
        req: Request<UpdateStatusRequest>,
    ) -> Result<Response<()>, Status> {
        let UpdateStatusRequest { emails, active } = req.into_inner();

        self.service
            .update_subscription_status(emails, active)
            .await
            .map_err(|e| Status::internal(format!("service error (update_subscription_status): {e}")))?;

        Ok(Response::new(()))
    }

    async fn delete(&self, req: Request<DeleteRequest>) -> Result<Response<()>, Status> {
        let emails = req.into_inner().emails;
        
        self.service
            .delete_subscriptions(emails)
            .await
            .map_err(|e| Status::internal(format!("service error (delete_subscriptions): {e}")))?;
        
        Ok(Response::new(()))
    }

    async fn demo(&self, req: Request<DemoRequest>) -> Result<Response<DemoResponse>, Status> {
        let test_email = req.into_inner().test_email;
        
        // Use a default email if none provided
        let email_to_use = if test_email.trim().is_empty() {
            "demo@example.com"
        } else {
            &test_email
        };
        
        match self.service.demo_functionality(email_to_use).await {
            Ok(demo_log) => {
                Ok(Response::new(DemoResponse {
                    demo_log,
                    success: true,
                }))
            },
            Err(e) => {
                let error_log = format!("Demo failed: {}\n\nThis demonstrates error handling in the trait-based dependency injection system.", e);
                Ok(Response::new(DemoResponse {
                    demo_log: error_log,
                    success: false,
                }))
            }
        }
    }
}
