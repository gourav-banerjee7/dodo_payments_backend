use axum::Router;
use dotenvy::dotenv;
use std::env;
use tower_http::trace::TraceLayer;
use tracing_subscriber::FmtSubscriber;

pub mod db;
pub mod routes;
pub mod handlers;
pub mod models;
pub mod utils;
pub mod middleware;
pub mod errors;

/// Initialize logging and return the Axum app
pub async fn create_app() -> Router {
    dotenv().ok();

    // Structured logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");

    let pool = db::connect().await;

    routes::create_routes(pool).layer(TraceLayer::new_for_http())
}
