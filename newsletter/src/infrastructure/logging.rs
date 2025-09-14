use serde_json::{json, Value};
use std::collections::HashMap;
use tracing::{info, error, warn, debug, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

/// Custom trace context for propagating trace IDs
#[derive(Debug, Clone)]
pub struct TraceContext {
    pub trace_id: String,
    pub span_id: String,
    pub operation: String,
}

impl TraceContext {
    pub fn new(operation: &str) -> Self {
        let trace_id = Uuid::new_v4().to_string();
        let span_id = Uuid::new_v4().to_string();
        
        Self {
            trace_id,
            span_id,
            operation: operation.to_string(),
        }
    }

    pub fn new_with_trace_id(trace_id: String, operation: &str) -> Self {
        let span_id = Uuid::new_v4().to_string();
        
        Self {
            trace_id,
            span_id,
            operation: operation.to_string(),
        }
    }
}

/// Initialize tracing with JSON formatting and trace context
pub fn init_tracing() -> anyhow::Result<()> {
    // Set up tracing subscriber with JSON formatting
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

/// Log structured JSON with trace context
pub fn log_with_trace(
    level: Level,
    trace_ctx: &TraceContext,
    message: &str,
    extra_fields: Option<HashMap<String, Value>>,
) {
    let mut fields = json!({
        "trace_id": trace_ctx.trace_id,
        "span_id": trace_ctx.span_id,
        "operation": trace_ctx.operation,
        "message": message,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "service": "newsletter"
    });

    if let Some(extra) = extra_fields {
        if let Value::Object(ref mut map) = fields {
            for (key, value) in extra {
                map.insert(key, value);
            }
        }
    }

    match level {
        Level::ERROR => error!("{}", fields),
        Level::WARN => warn!("{}", fields),
        Level::INFO => info!("{}", fields),
        Level::DEBUG => debug!("{}", fields),
        Level::TRACE => debug!("{}", fields),
    }
}

/// Log CRUD operation start
pub fn log_crud_start(trace_ctx: &TraceContext, operation: &str, entity: &str, details: Option<HashMap<String, Value>>) {
    let mut fields = HashMap::new();
    fields.insert("crud_operation".to_string(), json!(operation));
    fields.insert("entity".to_string(), json!(entity));
    fields.insert("status".to_string(), json!("start"));
    
    if let Some(details) = details {
        for (key, value) in details {
            fields.insert(key, value);
        }
    }

    log_with_trace(
        Level::INFO,
        trace_ctx,
        &format!("Starting {} operation on {}", operation, entity),
        Some(fields),
    );
}

/// Log CRUD operation success
pub fn log_crud_success(trace_ctx: &TraceContext, operation: &str, entity: &str, details: Option<HashMap<String, Value>>) {
    let mut fields = HashMap::new();
    fields.insert("crud_operation".to_string(), json!(operation));
    fields.insert("entity".to_string(), json!(entity));
    fields.insert("status".to_string(), json!("success"));
    
    if let Some(details) = details {
        for (key, value) in details {
            fields.insert(key, value);
        }
    }

    log_with_trace(
        Level::INFO,
        trace_ctx,
        &format!("Successfully completed {} operation on {}", operation, entity),
        Some(fields),
    );
}

/// Log CRUD operation error
pub fn log_crud_error(trace_ctx: &TraceContext, operation: &str, entity: &str, error: &str, details: Option<HashMap<String, Value>>) {
    let mut fields = HashMap::new();
    fields.insert("crud_operation".to_string(), json!(operation));
    fields.insert("entity".to_string(), json!(entity));
    fields.insert("status".to_string(), json!("error"));
    fields.insert("error".to_string(), json!(error));
    
    if let Some(details) = details {
        for (key, value) in details {
            fields.insert(key, value);
        }
    }

    log_with_trace(
        Level::ERROR,
        trace_ctx,
        &format!("Error in {} operation on {}: {}", operation, entity, error),
        Some(fields),
    );
}

/// Extract trace ID from tonic request headers
pub fn extract_trace_id_from_request<T>(request: &tonic::Request<T>) -> Option<String> {
    request
        .metadata()
        .get("x-trace-id")
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string())
}

/// Create trace context from request or generate new one
pub fn get_or_create_trace_context<T>(request: &tonic::Request<T>, operation: &str) -> TraceContext {
    if let Some(trace_id) = extract_trace_id_from_request(request) {
        TraceContext::new_with_trace_id(trace_id, operation)
    } else {
        TraceContext::new(operation)
    }
}