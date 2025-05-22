use axum::{
    Router,
    routing::{post, get},
    middleware::from_fn,
};
use sqlx::PgPool;

use crate::handlers::{user::*, transaction::*};
use crate::middleware::auth::auth;

pub fn create_routes(pool: PgPool) -> Router {
    let public_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login));

    let protected_routes = Router::new()
        .route("/transactions", post(create_transaction))
        .route("/balance", get(get_balance))
        .with_state(pool.clone()) // ✅ Explicit state needed
        .route_layer(from_fn(auth)); // ✅ Route layer after with_state

    public_routes
        .merge(protected_routes)
        .with_state(pool) // for public_routes
}
