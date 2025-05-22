use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    body::Body,
};
use uuid::Uuid;
use tracing::debug;
use crate::utils::jwt::verify_jwt;

pub async fn auth(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let sub = verify_jwt(auth_header).ok_or(StatusCode::UNAUTHORIZED)?;
    let user_id = Uuid::parse_str(&sub).map_err(|_| StatusCode::UNAUTHORIZED)?;

    debug!("✅ Authenticated user_id: {}", user_id);
    req.extensions_mut().insert(user_id);

    Ok(next.run(req).await)
}
