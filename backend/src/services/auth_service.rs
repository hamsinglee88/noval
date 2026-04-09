use bcrypt::{DEFAULT_COST, hash, verify};
use sqlx::SqlitePool;
use time::{Duration, OffsetDateTime, format_description::well_known::Rfc3339};
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::user::{AuthPayload, AuthUser, Credentials, SessionInfo, UserRow},
    validation::{validate_password, validate_username},
};

#[derive(Clone)]
pub struct AuthService {
    db: SqlitePool,
}

impl AuthService {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }

    pub async fn register(&self, credentials: Credentials) -> Result<AuthPayload, AppError> {
        validate_username(&credentials.username)?;
        validate_password(&credentials.password)?;

        let existing = sqlx::query_scalar::<_, String>("SELECT id FROM users WHERE username = ?")
            .bind(&credentials.username)
            .fetch_optional(&self.db)
            .await?;

        if existing.is_some() {
            return Err(AppError::conflict(
                "USERNAME_TAKEN",
                "用户名已存在，请更换后重试。",
            ));
        }

        let now = iso_timestamp_now();
        let expires_at = iso_timestamp_in_days(7);
        let session_token = Uuid::new_v4().to_string();
        let user_id = Uuid::new_v4().to_string();
        let password_hash = hash(&credentials.password, DEFAULT_COST)?;

        sqlx::query(
            "INSERT INTO users (
                id, username, password_hash, created_at, updated_at, last_login_at, session_token, session_expires_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&user_id)
        .bind(&credentials.username)
        .bind(&password_hash)
        .bind(&now)
        .bind(&now)
        .bind(&now)
        .bind(&session_token)
        .bind(&expires_at)
        .execute(&self.db)
        .await?;

        Ok(AuthPayload {
            user: AuthUser {
                id: user_id,
                username: credentials.username,
                created_at: now.clone(),
                last_login_at: Some(now),
            },
            session: SessionInfo {
                token: session_token,
                expires_at,
            },
        })
    }

    pub async fn login(&self, credentials: Credentials) -> Result<AuthPayload, AppError> {
        validate_username(&credentials.username)?;
        validate_password(&credentials.password)?;

        let user = sqlx::query_as::<_, UserRow>(
            "SELECT id, username, password_hash, created_at, updated_at, last_login_at, session_token, session_expires_at
             FROM users WHERE username = ?",
        )
        .bind(&credentials.username)
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| {
            AppError::unauthorized("INVALID_CREDENTIALS", "用户名或密码错误。")
        })?;

        if !verify(&credentials.password, &user.password_hash)? {
            return Err(AppError::unauthorized(
                "INVALID_CREDENTIALS",
                "用户名或密码错误。",
            ));
        }

        let session_token = Uuid::new_v4().to_string();
        let now = iso_timestamp_now();
        let expires_at = iso_timestamp_in_days(7);

        sqlx::query(
            "UPDATE users
             SET session_token = ?, session_expires_at = ?, last_login_at = ?, updated_at = ?
             WHERE id = ?",
        )
        .bind(&session_token)
        .bind(&expires_at)
        .bind(&now)
        .bind(&now)
        .bind(&user.id)
        .execute(&self.db)
        .await?;

        Ok(AuthPayload {
            user: AuthUser {
                id: user.id,
                username: user.username,
                created_at: user.created_at,
                last_login_at: Some(now),
            },
            session: SessionInfo {
                token: session_token,
                expires_at,
            },
        })
    }

    pub async fn logout(&self, session_token: &str) -> Result<(), AppError> {
        let result = sqlx::query(
            "UPDATE users
             SET session_token = NULL, session_expires_at = NULL, updated_at = ?
             WHERE session_token = ?",
        )
        .bind(iso_timestamp_now())
        .bind(session_token)
        .execute(&self.db)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::unauthorized(
                "INVALID_SESSION",
                "当前登录状态已失效，请重新登录。",
            ));
        }

        Ok(())
    }

    pub async fn current_user(&self, session_token: &str) -> Result<AuthPayload, AppError> {
        let user = sqlx::query_as::<_, UserRow>(
            "SELECT id, username, password_hash, created_at, updated_at, last_login_at, session_token, session_expires_at
             FROM users WHERE session_token = ?",
        )
        .bind(session_token)
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| AppError::unauthorized("INVALID_SESSION", "当前登录状态已失效，请重新登录。"))?;

        let expires_at = user
            .session_expires_at
            .clone()
            .ok_or_else(|| AppError::unauthorized("INVALID_SESSION", "当前登录状态已失效，请重新登录。"))?;

        let expires_at_value = OffsetDateTime::parse(&expires_at, &Rfc3339).map_err(|_| {
            AppError::unauthorized("INVALID_SESSION", "当前登录状态已失效，请重新登录。")
        })?;

        if expires_at_value <= OffsetDateTime::now_utc() {
            sqlx::query(
                "UPDATE users
                 SET session_token = NULL, session_expires_at = NULL, updated_at = ?
                 WHERE id = ?",
            )
            .bind(iso_timestamp_now())
            .bind(&user.id)
            .execute(&self.db)
            .await?;

            return Err(AppError::unauthorized(
                "SESSION_EXPIRED",
                "登录状态已过期，请重新登录。",
            ));
        }

        let token = user
            .session_token
            .clone()
            .ok_or_else(|| AppError::unauthorized("INVALID_SESSION", "当前登录状态已失效，请重新登录。"))?;

        Ok(AuthPayload {
            user: AuthUser::from(&user),
            session: SessionInfo {
                token,
                expires_at,
            },
        })
    }
}

fn iso_timestamp_now() -> String {
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .expect("rfc3339 timestamp")
}

fn iso_timestamp_in_days(days: i64) -> String {
    (OffsetDateTime::now_utc() + Duration::days(days))
        .format(&Rfc3339)
        .expect("rfc3339 timestamp")
}
