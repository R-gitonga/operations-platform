use chrono::{DateTime, Utc};
use sqlx::query_as;

use crate::{
    database::DbPool,
    models::partial_receiving_tracking::PartialReceivingTracking,
};

pub async fn find_active_by_wso_item(
    pool: &DbPool,
    wso_item_id: i32,
) -> Result<Option<PartialReceivingTracking>, sqlx::Error> {
    query_as::<_, PartialReceivingTracking>(
        r#"
        SELECT
            id,
            wso_item_id,
            first_partial_received_at,
            notification_sent_at,
            resolved_at,
            created_at,
            updated_at
        FROM partial_receiving_tracking
        WHERE wso_item_id = $1
          AND resolved_at IS NULL
        ORDER BY first_partial_received_at DESC
        LIMIT 1
        "#,
    )
    .bind(wso_item_id)
    .fetch_optional(pool)
    .await
}

pub async fn create(
    pool: &DbPool,
    wso_item_id: i32,
    first_partial_received_at: DateTime<Utc>,
) -> Result<PartialReceivingTracking, sqlx::Error> {
    query_as::<_, PartialReceivingTracking>(
        r#"
        INSERT INTO partial_receiving_tracking (
            wso_item_id,
            first_partial_received_at
        )
        VALUES ($1, $2)
        RETURNING
            id,
            wso_item_id,
            first_partial_received_at,
            notification_sent_at,
            resolved_at,
            created_at,
            updated_at
        "#,
    )
    .bind(wso_item_id)
    .bind(first_partial_received_at)
    .fetch_one(pool)
    .await
}

pub async fn mark_notification_sent(
    pool: &DbPool,
    id: i32,
    sent_at: DateTime<Utc>,
) -> Result<PartialReceivingTracking, sqlx::Error> {
    query_as::<_, PartialReceivingTracking>(
        r#"
        UPDATE partial_receiving_tracking
        SET
            notification_sent_at = $1,
            updated_at = NOW()
        WHERE id = $2
        RETURNING
            id,
            wso_item_id,
            first_partial_received_at,
            notification_sent_at,
            resolved_at,
            created_at,
            updated_at
        "#,
    )
    .bind(sent_at)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn resolve(
    pool: &DbPool,
    id: i32,
    resolved_at: DateTime<Utc>,
) -> Result<PartialReceivingTracking, sqlx::Error> {
    query_as::<_, PartialReceivingTracking>(
        r#"
        UPDATE partial_receiving_tracking
        SET
            resolved_at = $1,
            updated_at = NOW()
        WHERE id = $2
        RETURNING
            id,
            wso_item_id,
            first_partial_received_at,
            notification_sent_at,
            resolved_at,
            created_at,
            updated_at
        "#,
    )
    .bind(resolved_at)
    .bind(id)
    .fetch_one(pool)
    .await
}