use chrono::{DateTime, Utc};

use crate::database::DbPool;

pub async fn has_been_notified(
    pool: &DbPool,
    wso_item_id: i32,
    production_stage_id: i32,
    stage_started_at: DateTime<Utc>,
) -> Result<bool, sqlx::Error> {

    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM attention_required_notifications
            WHERE
                wso_item_id = $1
                AND production_stage_id = $2
                AND stage_started_at = $3
        )
        "#,
    )
    .bind(wso_item_id)
    .bind(production_stage_id)
    .bind(stage_started_at)
    .fetch_one(pool)
    .await?;

    Ok(exists)
}

pub async fn record_notification(
    pool: &DbPool,
    wso_item_id: i32,
    production_stage_id: i32,
    stage_started_at: DateTime<Utc>,
    notification_event_id: i32,
) -> Result<(), sqlx::Error> {

    sqlx::query(
        r#"
        INSERT INTO attention_required_notifications
        (
            wso_item_id,
            production_stage_id,
            stage_started_at,
            notification_event_id
        )
        VALUES
        (
            $1,
            $2,
            $3,
            $4
        )
        ON CONFLICT
        (
            wso_item_id,
            production_stage_id,
            stage_started_at
        )
        DO NOTHING
        "#,
    )
    .bind(wso_item_id)
    .bind(production_stage_id)
    .bind(stage_started_at)
    .bind(notification_event_id)
    .execute(pool)
    .await?;

    Ok(())
}