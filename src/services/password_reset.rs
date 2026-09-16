use chrono::{Duration, Utc};

use crate::{
    config::Config,
    database::DbPool,
    errors::app_error::AppError,
    models::email_message::EmailMessage,
    repositories::{password_reset_token, user},
    services::{email_sender, password, user_rules},
};

const RESET_TOKEN_LIFETIME_MINUTES: i64 = 30;

pub async fn request(
    pool: &DbPool,
    config: &Config,
    email: &str,
) -> Result<(), AppError> {
    let email = email.trim().to_lowercase();

    if email.is_empty() {
        return Ok(());
    }

    let user = match user::find_by_email(pool, &email).await {
        Ok(user) if user.active => user,
        Ok(_) | Err(sqlx::Error::RowNotFound) => return Ok(()),
        Err(error) => return Err(error.into()),
    };

    let token = password::generate_reset_token();
    let token_hash = password::hash_password(&token)
        .map_err(AppError::Validation)?;

    password_reset_token::invalidate_for_user(pool, user.id).await?;
    password_reset_token::create(
        pool,
        user.id,
        &token_hash,
        Utc::now() + Duration::minutes(RESET_TOKEN_LIFETIME_MINUTES),
    )
    .await?;

    let reset_url = format!(
        "{}/reset-password?token={}",
        std::env::var("FRONTEND_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:3000".to_string())
            .trim_end_matches('/'),
        token,
    );

    let message = EmailMessage {
        from_name: config.system_notification_name.clone(),
        from_email: config.system_notification_email.clone(),
        to: user.email,
        subject: "Reset your Operations Platform password".to_string(),
        html_body: format!(
            "<p>A password reset was requested for your Operations Platform account.</p><p><a href=\"{reset_url}\">Reset your password</a></p><p>This link expires in 30 minutes. If you did not request a reset, you can ignore this email.</p>",
        ),
    };

    if let Err(error) = email_sender::send(config, message).await {
        eprintln!("Failed to send password reset email: {error}");
    }

    Ok(())
}

pub async fn reset(
    pool: &DbPool,
    token: &str,
    new_password: &str,
) -> Result<(), AppError> {
    user_rules::ensure_password_valid(new_password)?;

    let tokens = password_reset_token::find_active(pool).await?;
    let matching_token = tokens.into_iter().find(|candidate| {
        password::verify_password(token, &candidate.token_hash).unwrap_or(false)
    });

    let token = matching_token.ok_or_else(|| {
        AppError::Validation("This password reset link is invalid or has expired.".to_string())
    })?;

    let password_hash = password::hash_password(new_password)
        .map_err(AppError::Validation)?;

    let consumed = password_reset_token::consume_and_update_password(
        pool,
        token.id,
        token.user_id,
        &password_hash,
    )
    .await?;

    if !consumed {
        return Err(AppError::Validation(
            "This password reset link is invalid or has expired.".to_string(),
        ));
    }

    Ok(())
}
