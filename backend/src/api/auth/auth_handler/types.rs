use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct UserCred {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub is_active: i32,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub dob: Option<String>,
    pub bio: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub id: i64,
    pub email: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// Can be email or username
    pub identifier: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user_id: i64,
    pub username: String,
    pub email: String,
}
