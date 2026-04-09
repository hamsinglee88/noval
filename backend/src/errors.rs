use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{message}")]
    BadRequest {
        code: &'static str,
        message: String,
    },
    #[error("{message}")]
    Conflict {
        code: &'static str,
        message: String,
    },
    #[error("{message}")]
    Unauthorized {
        code: &'static str,
        message: String,
    },
    #[error("{message}")]
    NotFound {
        code: &'static str,
        message: String,
    },
    #[error("{message}")]
    Internal {
        code: &'static str,
        message: String,
    },
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    success: bool,
    error: ErrorDetail,
}

#[derive(Debug, Serialize)]
struct ErrorDetail {
    code: &'static str,
    message: String,
}

impl AppError {
    pub fn bad_request(code: &'static str, message: impl Into<String>) -> Self {
        Self::BadRequest {
            code,
            message: message.into(),
        }
    }

    pub fn conflict(code: &'static str, message: impl Into<String>) -> Self {
        Self::Conflict {
            code,
            message: message.into(),
        }
    }

    pub fn unauthorized(code: &'static str, message: impl Into<String>) -> Self {
        Self::Unauthorized {
            code,
            message: message.into(),
        }
    }

    pub fn not_found(code: &'static str, message: impl Into<String>) -> Self {
        Self::NotFound {
            code,
            message: message.into(),
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            code: "INTERNAL_SERVER_ERROR",
            message: message.into(),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest { .. } => StatusCode::BAD_REQUEST,
            Self::Conflict { .. } => StatusCode::CONFLICT,
            Self::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
            Self::NotFound { .. } => StatusCode::NOT_FOUND,
            Self::Internal { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::BadRequest { code, .. }
            | Self::Conflict { code, .. }
            | Self::Unauthorized { code, .. }
            | Self::NotFound { code, .. }
            | Self::Internal { code, .. } => code,
        }
    }

    fn message(&self) -> String {
        self.to_string()
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = Json(ErrorResponse {
            success: false,
            error: ErrorDetail {
                code: self.code(),
                message: self.message(),
            },
        });

        (status, body).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        // 记录详细错误到日志，但不返回给客户端
        error!("Database error: {:?}", value);
        Self::internal("数据库操作失败，请稍后重试。")
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(value: bcrypt::BcryptError) -> Self {
        // 记录详细错误到日志，但不返回给客户端
        error!("Password hashing error: {:?}", value);
        Self::internal("密码处理失败，请稍后重试。")
    }
}

impl From<axum::extract::multipart::MultipartError> for AppError {
    fn from(value: axum::extract::multipart::MultipartError) -> Self {
        error!("Multipart error: {:?}", value);
        Self::bad_request(
            "MULTIPART_ERROR",
            "文件上传失败，请重试。",
        )
    }
}
