use axum::{Json, extract::{State, Extension}};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Utc, NaiveDateTime};
use bigdecimal::BigDecimal;
use crate::models::Transaction;
use validator::Validate;
use crate::errors::AppError;
use axum::debug_handler;

#[derive(Deserialize, Validate)]
pub struct TransactionRequest {
    pub amount: BigDecimal,
    #[validate(length(max = 255))]
    pub description: Option<String>,
}

#[debug_handler]
pub async fn create_transaction(
    State(pool): State<PgPool>,
    Extension(user_id): Extension<Uuid>,
    Json(payload): Json<TransactionRequest>
) -> Result<Json<Transaction>, AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    if payload.amount <= BigDecimal::from(0) {
        return Err(AppError::ValidationError("Amount must be greater than 0".to_string()));
    }

    let id = Uuid::new_v4();
    let now = Utc::now().naive_utc();

    let txn = sqlx::query_as!(Transaction,
        "INSERT INTO transactions (id, user_id, amount, description, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        id, user_id, payload.amount, payload.description, now
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(Json(txn))
}

#[derive(Serialize)]
pub struct BalanceResponse {
    pub user_id: Uuid,
    pub balance: BigDecimal,
}

#[debug_handler]
pub async fn get_balance(
    State(pool): State<PgPool>,
    Extension(user_id): Extension<Uuid>
) -> Result<Json<BalanceResponse>, AppError> {
    let row = sqlx::query!(
        "SELECT COALESCE(SUM(amount), 0) as balance FROM transactions WHERE user_id = $1",
        user_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(BalanceResponse {
        user_id,
        balance: row.balance.unwrap_or_default(),
    }))
}
