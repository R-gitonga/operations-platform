use std::env;

use crate::errors::app_error::AppError;

#[derive(Clone)]
pub struct Config {
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,

    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_from: String,

    pub system_notification_name: String,
    pub system_notification_email: String,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        // ---------------------------------------------------------
        // JWT configuration
        // ---------------------------------------------------------

        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| AppError::Validation("JWT_SECRET is not configured.".into()))?;

        if jwt_secret.trim().is_empty() {
            return Err(AppError::Validation("JWT_SECRET cannot be empty.".into()));
        }

        let jwt_expiration_hours = env::var("JWT_EXPIRATION_HOURS")
            .unwrap_or_else(|_| "8".to_string())
            .parse::<i64>()
            .map_err(|_| {
                AppError::Validation("JWT_EXPIRATION_HOURS must be a valid number.".into())
            })?;

        if jwt_expiration_hours <= 0 {
            return Err(AppError::Validation(
                "JWT_EXPIRATION_HOURS must be greater than zero.".into(),
            ));
        }

        // ---------------------------------------------------------
        // SMTP configuration
        // ---------------------------------------------------------

        let smtp_host = env::var("SMTP_HOST")
            .map_err(|_| AppError::Validation("SMTP_HOST is not configured.".into()))?;

        if smtp_host.trim().is_empty() {
            return Err(AppError::Validation("SMTP_HOST cannot be empty.".into()));
        }

        let smtp_port = env::var("SMTP_PORT")
            .unwrap_or_else(|_| "587".to_string())
            .parse::<u16>()
            .map_err(|_| AppError::Validation("SMTP_PORT must be a valid port number.".into()))?;

        let smtp_username = env::var("SMTP_USERNAME")
            .map_err(|_| AppError::Validation("SMTP_USERNAME is not configured.".into()))?;

        if smtp_username.trim().is_empty() {
            return Err(AppError::Validation(
                "SMTP_USERNAME cannot be empty.".into(),
            ));
        }

        let smtp_password = env::var("SMTP_PASSWORD")
            .map_err(|_| AppError::Validation("SMTP_PASSWORD is not configured.".into()))?;

        if smtp_password.is_empty() {
            return Err(AppError::Validation(
                "SMTP_PASSWORD cannot be empty.".into(),
            ));
        }

        let smtp_from = env::var("SMTP_FROM")
            .map_err(|_| AppError::Validation("SMTP_FROM is not configured.".into()))?;

        if smtp_from.trim().is_empty() {
            return Err(AppError::Validation("SMTP_FROM cannot be empty.".into()));
        }

        let system_notification_name = env::var("SYSTEM_NOTIFICATION_NAME").map_err(|_| {
            AppError::Validation("SYSTEM_NOTIFICATION_NAME is not configured.".into())
        })?;

        if system_notification_name.trim().is_empty() {
            return Err(AppError::Validation(
                "SYSTEM_NOTIFICATION_NAME cannot be empty.".into(),
            ));
        }

        let system_notification_email = env::var("SYSTEM_NOTIFICATION_EMAIL").map_err(|_| {
            AppError::Validation("SYSTEM_NOTIFICATION_EMAIL is not configured.".into())
        })?;

        if system_notification_email.trim().is_empty() {
            return Err(AppError::Validation(
                "SYSTEM_NOTIFICATION_EMAIL cannot be empty.".into(),
            ));
        }

        // ---------------------------------------------------------
        // Final configuration
        // ---------------------------------------------------------

        Ok(Self {
            jwt_secret,
            jwt_expiration_hours,

            smtp_host,
            smtp_port,
            smtp_username,
            smtp_password,
            smtp_from,
            system_notification_name,
            system_notification_email,
        })
    }
}
