use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use clap::Parser;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber;

mod proxyd;

use anyhow::Result;
use proxyd::cli::DaemonCli;

use crate::proxyd::{models::AnthropicRequest, proxy::ProxyHandler};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = DaemonCli::parse();
    let config = proxyd::config::load_config(&cli.config)?;

    info!("Starting ccv-d proxy server on {}", config.server.bind_addr);

    let proxy_handler = Arc::new(ProxyHandler::new(config.providers, config.proxy));
    let app = create_app(proxy_handler);

    let addr = config.server.bind_addr;
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("ccv-d proxy server listening on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

fn create_app(proxy_handler: Arc<ProxyHandler>) -> Router {
    Router::new()
        .route("/v1/messages", post(chat_completions))
        .route("/v1/models", get(list_models))
        .layer(CorsLayer::permissive())
        .with_state(proxy_handler)
}

async fn chat_completions(
    State(handler): State<Arc<ProxyHandler>>,
    Json(payload): Json<AnthropicRequest>,
) -> Result<axum::response::Response, StatusCode> {
    todo!()
}

async fn list_models(
    State(handler): State<Arc<ProxyHandler>>,
) -> Result<axum::response::Response, StatusCode> {
    todo!()
}
