use std::{env, net::SocketAddr};
use tonic::transport::Server;
use tonic_reflection::server::Builder as ReflBuilder;

use infrastructure::db::{build_pool, run_migrations, PgPool};
use infrastructure::rpc::newsletter::v1::proto::newsletter_service_server::NewsletterServiceServer;
use infrastructure::rpc::newsletter::v1::{api::MyNewsletterService, proto};

use tracing::{error, info};
use tracing_subscriber::{fmt, EnvFilter};

mod domain;
mod infrastructure;
mod repository;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env (optional)
    dotenv::dotenv().ok();

    // ---------- JSON logging (tracing) ----------
    // Requires tracing-subscriber features: "fmt", "json", "env-filter".
    // Example: RUST_LOG=info,hyper=warn
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    fmt()
        .json() // newline-delimited JSON
        .flatten_event(true) // put event fields at top-level
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_env_filter(env_filter)
        .init();

    // ---------- DB: pool + migrations ----------
    let _pool: PgPool = build_pool().await?;
    run_migrations().await?; // matches signature that reads DATABASE_URL internally

    // ---------- Address ----------
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(50051);
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;

    // ---------- Reflection (v1) ----------
    // Requires FILE_DESCRIPTOR_SET exposed from proto module and build.rs generating it.
    let reflection = ReflBuilder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()?; // tonic-reflection 0.14 uses build_v1()/build_v1alpha()

    info!(message = "Starting gRPC server", %host, %port);

    // ---------- Service ----------
    let pool = build_pool().await?;
    run_migrations().await?;
    let service = MyNewsletterService::new(pool.clone());

    // ---------- Graceful shutdown ----------
    // Standard tonic + Tokio signal pattern.
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

        info!("Shutdown signal received, stopping gRPC server gracefully...");
    };

    // ---------- Server ----------
    Server::builder()
        .add_service(reflection)
        .add_service(NewsletterServiceServer::new(service))
        .serve_with_shutdown(addr, shutdown)
        .await?; // let anyhow convert tonic::transport::Error

    info!("Server stopped");
    Ok(())
}
