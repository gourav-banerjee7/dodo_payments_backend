use axum::{Json, extract::State};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Utc, NaiveDateTime};
use crate::models::User;
use crate::utils::hash::{hash_password, verify_password};
use crate::utils::jwt::create_jwt;
use validator::Validate;
use crate::errors::AppError;
use axum::debug_handler;

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[debug_handler]
pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterRequest>
) -> Result<Json<User>, AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    // Check if email already exists
    let existing_user = sqlx::query!(
        "SELECT id FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        println!("Database query failed: {:?}", e); // Debug
        AppError::DatabaseError(e.to_string())
    })?;

    if existing_user.is_some() {
        println!("Duplicate email detected: {}", payload.email); // Debug
        return Err(AppError::Conflict("Email already registered".to_string()));
    }

    let id = Uuid::new_v4();
    let now = Utc::now().naive_utc();
    let password_hash = hash_password(&payload.password);

    let user = sqlx::query_as!(User,
        "INSERT INTO users (id, email, password_hash, created_at) VALUES ($1, $2, $3, $4) RETURNING *",
        id, payload.email, password_hash, now
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(Json(user))
}

#[debug_handler]
pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>
) -> Result<Json<LoginResponse>, AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let user = sqlx::query_as!(User,
        "SELECT * FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    if let Some(user) = user {
        if verify_password(&user.password_hash, &payload.password) {
            let token = create_jwt(&user.id.to_string());
            return Ok(Json(LoginResponse { token }));
        }
    }

    Err(AppError::Unauthorized)
}

