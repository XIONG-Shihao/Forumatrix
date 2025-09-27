use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct UserCred {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub is_active: i32,
}
