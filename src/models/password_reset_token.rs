use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct PasswordResetToken {
    pub id: i32,
    pub user_id: i32,
    pub token_hash: String,
}
