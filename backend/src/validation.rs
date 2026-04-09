use regex::Regex;

use crate::errors::AppError;

const USERNAME_PATTERN: &str = r"^[A-Za-z0-9_]{3,20}$";

pub fn validate_username(username: &str) -> Result<(), AppError> {
    let regex = Regex::new(USERNAME_PATTERN).expect("valid username regex");
    if regex.is_match(username) {
        Ok(())
    } else {
        Err(AppError::bad_request(
            "INVALID_USERNAME",
            "用户名需为 3-20 位，仅允许字母、数字和下划线。",
        ))
    }
}

pub fn validate_password(password: &str) -> Result<(), AppError> {
    let has_letter = password.chars().any(|char| char.is_ascii_alphabetic());
    let has_digit = password.chars().any(|char| char.is_ascii_digit());
    if password.len() >= 8 && has_letter && has_digit {
        Ok(())
    } else {
        Err(AppError::bad_request(
            "WEAK_PASSWORD",
            "密码至少 8 位，且必须同时包含字母和数字。",
        ))
    }
}
