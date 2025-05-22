use serde::{Serialize, Deserialize};
use uuid::Uuid;
use sqlx::types::BigDecimal;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub amount: BigDecimal,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}
