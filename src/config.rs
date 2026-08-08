use std::env;

use crate::errors::app_error::AppError;

#[derive(Clone)]
pub struct Config {
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        let jwt_secret =
            env::var("JWT_SECRET")
                .map_err(|_| {
                    AppError::Validation(
                        "JWT_SECRET is not configured.".into(),
                    )
                })?;

        if jwt_secret.trim().is_empty() {
            return Err(
                AppError::Validation(
                    "JWT_SECRET cannot be empty.".into(),
                ),
            );
        }

        let jwt_expiration_hours =
            env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "8".to_string())
                .parse::<i64>()
                .map_err(|_| {
                    AppError::Validation(
                        "JWT_EXPIRATION_HOURS must be a valid number.".into(),
                    )
                })?;

        if jwt_expiration_hours <= 0 {
            return Err(
                AppError::Validation(
                    "JWT_EXPIRATION_HOURS must be greater than zero.".into(),
                ),
            );
        }

        Ok(Self {
            jwt_secret,
            jwt_expiration_hours,
        })
    }
}