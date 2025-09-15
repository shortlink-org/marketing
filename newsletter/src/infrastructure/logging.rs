use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Initialize tracing with JSON formatting
pub fn init_tracing() -> anyhow::Result<()> {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .flatten_event(true)
                .with_target(true)
                .with_file(true)
                .with_line_number(true)
                .with_current_span(true)
                .with_span_list(false),
        )
        .init();

    Ok(())
}

/// Extract trace ID from tonic request headers
pub fn extract_trace_id_from_request<T>(request: &tonic::Request<T>) -> Option<String> {
    request
        .metadata()
        .get("x-trace-id")
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string())
}