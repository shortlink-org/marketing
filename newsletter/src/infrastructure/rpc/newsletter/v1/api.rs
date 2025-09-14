use async_trait::async_trait;
use tonic::{Request, Response, Status};

use crate::infrastructure::rpc::newsletter::v1::proto::{
    newsletter_service_server::NewsletterService, DeleteRequest, GetRequest, GetResponse,
    ListResponse, SubscribeRequest, UnSubscribeRequest, UpdateStatusRequest,
};

#[derive(Default)]
pub struct MyNewsletterService;

#[async_trait]
impl NewsletterService for MyNewsletterService {
    async fn get(&self, req: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let email = req.into_inner().email;
        Ok(Response::new(GetResponse {
            email,
            active: true,
        }))
    }
    async fn subscribe(&self, req: Request<SubscribeRequest>) -> Result<Response<()>, Status> {
        println!("Subscribed: {}", req.into_inner().email);
        Ok(Response::new(()))
    }
    async fn un_subscribe(&self, req: Request<UnSubscribeRequest>) -> Result<Response<()>, Status> {
        println!("Unsubscribed: {}", req.into_inner().email);
        Ok(Response::new(()))
    }
    async fn list(&self, _req: Request<()>) -> Result<Response<ListResponse>, Status> {
        Ok(Response::new(ListResponse {
            newsletters: vec![],
        }))
    }
    async fn update_status(
        &self,
        req: Request<UpdateStatusRequest>,
    ) -> Result<Response<()>, Status> {
        let req = req.into_inner();
        for email in req.emails {
            println!("Update {} -> {}", email, req.active);
        }
        Ok(Response::new(()))
    }
    async fn delete(&self, req: Request<DeleteRequest>) -> Result<Response<()>, Status> {
        for email in req.into_inner().emails {
            println!("Delete {}", email);
        }
        Ok(Response::new(()))
    }
}
