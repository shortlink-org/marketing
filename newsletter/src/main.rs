use std::{env, net::SocketAddr};
use tonic::transport::Server;
use tonic_reflection::server::Builder as ReflBuilder;

use infrastructure::rpc::newsletter::v1::proto::newsletter_service_server::NewsletterServiceServer;
use infrastructure::rpc::newsletter::v1::{
    api::MyNewsletterService,
    proto, // for FILE_DESCRIPTOR_SET
};

mod infrastructure;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Resolve bind address from env (defaults to 0.0.0.0:50051)
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(50051);
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;

    // Build gRPC Server Reflection service using the compiled descriptor set.
    // NOTE: build_v1() is the API in tonic-reflection 0.14.
    let reflection = ReflBuilder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()
        .expect("failed to build reflection service"); // docs show v1/v1alpha builders

    println!("NewsletterService gRPC server listening on {}", addr);

    // Instantiate service implementation
    let service = MyNewsletterService::default();

    // Build a shutdown future that triggers on Ctrl+C (and SIGTERM on Unix)
    let shutdown = async {
        let ctrl_c = async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let sigterm = async {
            use tokio::signal::unix::{signal, SignalKind};
            let mut term =
                signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");
            term.recv().await;
        };

        #[cfg(not(unix))]
        let sigterm = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {},
            _ = sigterm => {},
        }

        println!("Shutdown signal received, stopping gRPC server gracefully...");
    };

    // Start gRPC server with reflection and graceful shutdown
    Server::builder() // tonic::transport::Server builder
        .add_service(reflection) // reflection service
        .add_service(NewsletterServiceServer::new(service)) // your gRPC service
        .serve_with_shutdown(addr, shutdown) // graceful shutdown pattern
        .await?;

    Ok(())
}
