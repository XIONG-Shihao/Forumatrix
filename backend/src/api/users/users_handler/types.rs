use serde::Serialize;

/// Public-safe user fields (no password hash).
#[derive(Debug, Clone, Serialize)]
pub struct UserPublic {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub dob: Option<String>,
    pub bio: Option<String>,
    pub is_active: i32,
    pub is_admin: i32,
}
