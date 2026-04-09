use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct UserRow {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: String,
    pub updated_at: String,
    pub last_login_at: Option<String>,
    pub session_token: Option<String>,
    pub session_expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct AuthUser {
    pub id: String,
    pub username: String,
    pub created_at: String,
    pub last_login_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SessionInfo {
    pub token: String,
    pub expires_at: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct AuthPayload {
    pub user: AuthUser,
    pub session: SessionInfo,
}

impl From<&UserRow> for AuthUser {
    fn from(value: &UserRow) -> Self {
        Self {
            id: value.id.clone(),
            username: value.username.clone(),
            created_at: value.created_at.clone(),
            last_login_at: value.last_login_at.clone(),
        }
    }
}
