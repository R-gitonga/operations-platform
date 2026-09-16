use chrono::{DateTime, Utc};
use sqlx::{query, query_as};

use crate::{
    database::DbPool,
    models::password_reset_token::PasswordResetToken,
};

pub async fn invalidate_for_user(
    pool: &DbPool,
    user_id: i32,
) -> Result<(), sqlx::Error> {
    query(
        "UPDATE password_reset_tokens SET used_at = NOW() WHERE user_id = $1 AND used_at IS NULL",
    )
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create(
    pool: &DbPool,
    user_id: i32,
    token_hash: &str,
    expires_at: DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    query(
        "INSERT INTO password_reset_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
    )
    .bind(user_id)
    .bind(token_hash)
    .bind(expires_at)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn find_active(
    pool: &DbPool,
) -> Result<Vec<PasswordResetToken>, sqlx::Error> {
    query_as::<_, PasswordResetToken>(
        "SELECT id, user_id, token_hash FROM password_reset_tokens WHERE used_at IS NULL AND expires_at > NOW()",
    )
    .fetch_all(pool)
    .await
}

pub async fn consume_and_update_password(
    pool: &DbPool,
    token_id: i32,
    user_id: i32,
    password_hash: &str,
) -> Result<bool, sqlx::Error> {
    let mut transaction = pool.begin().await?;

    let token = query(
        "UPDATE password_reset_tokens SET used_at = NOW() WHERE id = $1 AND user_id = $2 AND used_at IS NULL AND expires_at > NOW() RETURNING id",
    )
    .bind(token_id)
    .bind(user_id)
    .fetch_optional(&mut *transaction)
    .await?;

    if token.is_none() {
        transaction.rollback().await?;
        return Ok(false);
    }

    query(
        "UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2",
    )
    .bind(password_hash)
    .bind(user_id)
    .execute(&mut *transaction)
    .await?;

    query(
        "UPDATE password_reset_tokens SET used_at = NOW() WHERE user_id = $1 AND used_at IS NULL",
    )
    .bind(user_id)
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Ok(true)
}
