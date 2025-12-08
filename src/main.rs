use axum::{
    routing::{get, post},
    Router,
};

use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

mod routes;
mod models;
mod services;

#[tokio::main]
async fn main() {
    // Setup tracing / logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Build router
    let app = Router::new()
        .route("/health", get(routes::health))
        .route("/analyze_tx", post(routes::analyze_tx));

    // Bind address
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("🚀 AI Blockchain Analyzer listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind");

    axum::serve(listener, app)
        .await
        .expect("server failed");
}
