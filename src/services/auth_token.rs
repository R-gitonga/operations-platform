use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode,
    encode,
    DecodingKey,
    EncodingKey,
    Header,
    Validation,
};
use serde::{Deserialize, Serialize};

use crate::errors::app_error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    pub sub: i32,
    pub exp: usize,
}

pub fn create_token(
    user_id: i32,
    secret: &str,
    duration_hours: i64,
) -> Result<String, AppError> {
    let expiration =
        Utc::now()
            .checked_add_signed(
                Duration::hours(duration_hours),
            )
            .ok_or_else(|| {
                AppError::Validation(
                    "Unable to calculate token expiration.".into(),
                )
            })?;

    let claims = AuthClaims {
        sub: user_id,
        exp: expiration.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(
            secret.as_bytes(),
        ),
    )
    .map_err(|error| {
        AppError::Validation(
            format!(
                "Failed to create authentication token: {}",
                error
            ),
        )
    })
}

pub fn validate_token(
    token: &str,
    secret: &str,
) -> Result<AuthClaims, AppError> {
    let token_data =
        decode::<AuthClaims>(
            token,
            &DecodingKey::from_secret(
                secret.as_bytes(),
            ),
            &Validation::default(),
        )
        .map_err(|_| {
            AppError::Validation(
                "Invalid or expired authentication token.".into(),
            )
        })?;

    Ok(token_data.claims)
}