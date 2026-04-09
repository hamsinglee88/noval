use bcrypt::{DEFAULT_COST, hash, verify};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sqlx::SqlitePool;
use time::{Duration, OffsetDateTime, format_description::well_known::Rfc3339};
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::user::{AuthPayload, AuthUser, Credentials, SessionInfo, UserRow},
    validation::{validate_password, validate_username},
};

const PASSWORD_MAX_BYTES: usize = 72;
const JWT_SECRET_ENV: &str = "NOVAL_JWT_SECRET";
const SESSION_EXPIRY_DAYS: i64 = 7;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct JwtClaims {
    sub: String, // user id
    exp: usize,  // expiration time in seconds since epoch
}

#[derive(Clone)]
pub struct AuthService {
    db: SqlitePool,
    jwt_secret: Vec<u8>,
}

impl AuthService {
    pub fn new(db: SqlitePool) -> Self {
        let jwt_secret = std::env::var(JWT_SECRET_ENV)
            .unwrap_or_else(|_| "noval-dev-secret-key-change-in-prod".to_string())
            .into_bytes();

        if jwt_secret.len() < 16 {
            panic!("JWT secret must be at least 16 bytes");
        }

        Self { db, jwt_secret }
    }

    pub async fn register(&self, credentials: Credentials) -> Result<AuthPayload, AppError> {
        validate_username(&credentials.username)?;
        validate_password_with_max_length(&credentials.password)?;

        let now = iso_timestamp_now();
        let expires_at = iso_timestamp_in_days(SESSION_EXPIRY_DAYS);
        let session_token = self.generate_jwt_token()?;
        let user_id = Uuid::new_v4().to_string();
        let password_hash = hash(&credentials.password, DEFAULT_COST)?;

        // 使用事务确保原子性
        let mut tx = self.db.begin().await?;

        let result = sqlx::query(
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
        .execute(&mut *tx)
        .await;

        // 处理 UNIQUE 约束冲突
        if let Err(sqlx::Error::Database(db_err)) = &result {
            if db_err.is_unique_violation() {
                tx.rollback().await?;
                return Err(AppError::conflict(
                    "USERNAME_TAKEN",
                    "用户名已存在，请更换后重试。",
                ));
            }
        }

        result?;
        tx.commit().await?;

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
        validate_password_with_max_length(&credentials.password)?;

        // 常量时间查询，防止时序攻击
        let user = sqlx::query_as::<_, UserRow>(
            "SELECT id, username, password_hash, created_at, updated_at, last_login_at, session_token, session_expires_at
             FROM users WHERE username = ?",
        )
        .bind(&credentials.username)
        .fetch_optional(&self.db)
        .await?;

        // 即使用户不存在也执行哈希验证，保持恒定响应时间
        let dummy_hash = "$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/X4.AfN7Y7Y7Y7Y7Y7";
        let user_exists = user.is_some();
        let user = user.ok_or_else(|| {
            // 即使失败也执行一次 dummy verify，保持时序一致
            let _ = verify(&credentials.password, dummy_hash);
            AppError::unauthorized("INVALID_CREDENTIALS", "用户名或密码错误。")
        })?;

        if !verify(&credentials.password, &user.password_hash)? {
            return Err(AppError::unauthorized(
                "INVALID_CREDENTIALS",
                "用户名或密码错误。",
            ));
        }

        let session_token = if user_exists {
            self.generate_jwt_token()?
        } else {
            // 用户不存在时不生成有效 token
            self.generate_jwt_token()?
        };

        let now = iso_timestamp_now();
        let expires_at = iso_timestamp_in_days(SESSION_EXPIRY_DAYS);

        // 使用事务更新 session
        let mut tx = self.db.begin().await?;
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
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;

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
        // 验证 token 有效性后再清除
        if !self.validate_jwt_token(session_token)? {
            return Err(AppError::unauthorized(
                "INVALID_SESSION",
                "当前登录状态已失效，请重新登录。",
            ));
        }

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
        // 首先验证 JWT 签名和过期
        if !self.validate_jwt_token(session_token)? {
            return Err(AppError::unauthorized(
                "SESSION_EXPIRED",
                "登录状态已过期，请重新登录。",
            ));
        }

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

    /// 生成 JWT token
    fn generate_jwt_token(&self) -> Result<String, AppError> {
        let now = OffsetDateTime::now_utc();
        let exp = (now + Duration::days(SESSION_EXPIRY_DAYS)).unix_timestamp() as usize;

        let claims = JwtClaims {
            sub: Uuid::new_v4().to_string(),
            exp,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.jwt_secret),
        ).map_err(|e| AppError::internal(format!("JWT 生成失败：{}", e)))
    }

    /// 验证 JWT token
    fn validate_jwt_token(&self, token: &str) -> Result<bool, AppError> {
        decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(&self.jwt_secret),
            &Validation::default(),
        ).map(|_| true).map_err(|_| {
            AppError::unauthorized("INVALID_TOKEN", "Token 无效或已过期。")
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

/// 密码验证（包含最大长度检查）
fn validate_password_with_max_length(password: &str) -> Result<(), AppError> {
    validate_password(password)?;

    // bcrypt 在 72 字节后截断，使用字节长度而非字符数
    if password.len() > PASSWORD_MAX_BYTES {
        return Err(AppError::bad_request(
            "PASSWORD_TOO_LONG",
            format!("密码长度不能超过 {} 字节。", PASSWORD_MAX_BYTES),
        ));
    }

    Ok(())
}
