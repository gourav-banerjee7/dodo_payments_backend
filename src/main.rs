use dodo_payments_backend::create_app;
use std::env;

#[tokio::main]
async fn main() {
    let app = create_app().await;

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("{}:{}", host, port);

    tracing::info!("🚀 Server running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app.into_make_service())
        .await
        .unwrap();
}
