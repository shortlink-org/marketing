# Multi-stage build for Rust newsletter service
FROM rust:1.93-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY build.rs ./

# Copy source code
COPY src ./src
COPY templates ./templates

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim as runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false newsletter

# Set working directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/newsletter /usr/local/bin/newsletter

# Copy templates
COPY --from=builder /app/templates ./templates

# Change ownership
RUN chown -R newsletter:newsletter /app

# Switch to app user
USER newsletter

# Expose port
EXPOSE 50051

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD grpc_health_probe -addr=localhost:50051 || exit 1

# Set environment variables
ENV RUST_LOG=info
ENV HOST=0.0.0.0
ENV PORT=50051

# Run the binary
CMD ["newsletter"]