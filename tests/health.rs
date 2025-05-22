use axum::http::{Request, StatusCode};
use tower::util::ServiceExt;
mod common;

#[tokio::test]
async fn test_balance_unauthorized() {
    let app = common::spawn_app().await;

    let response = app
        .oneshot(Request::builder().uri("/balance").body(axum::body::Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
