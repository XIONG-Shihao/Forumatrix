use sqlx::FromRow;

/// Row shape used by SQLx for queries (no password hash).
#[derive(Debug, Clone, FromRow)]
pub struct UserRow {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub dob: Option<String>,
    pub bio: Option<String>,
    pub is_active: i32,
    pub is_admin: i32,
}
