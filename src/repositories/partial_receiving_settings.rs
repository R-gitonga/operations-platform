
use sqlx::query_as;

use crate::{
    database::DbPool,
    models::partial_receiving_settings::PartialReceivingSettings,
};

pub async fn get(
    pool: &DbPool,
) -> Result<PartialReceivingSettings, sqlx::Error> {
    sqlx::query_as::<_, PartialReceivingSettings>(
        r#"
        SELECT
            id,
            attention_after_days,
            updated_at
        FROM partial_receiving_settings
        WHERE id = 1
        "#,
    )
    .fetch_one(pool)
    .await
}

pub async fn update_attention_after_days(
    pool: &DbPool,
    days: i32,
) -> Result<PartialReceivingSettings, sqlx::Error> {
    sqlx::query_as::<_, PartialReceivingSettings>(
        r#"
        UPDATE partial_receiving_settings
        SET
            attention_after_days = $1,
            updated_at = NOW()
        WHERE id = 1
        RETURNING
            id,
            attention_after_days,
            updated_at
        "#,
    )
    .bind(days)
    .fetch_one(pool)
    .await
}