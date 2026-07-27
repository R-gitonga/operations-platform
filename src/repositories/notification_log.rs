use crate::{
    database::DbPool,
    models::notification_log::NotificationLog,
};

use sqlx::Row;

pub async fn create_pending(
    pool: &DbPool,
    notification_event_id: i32,
    recipient_email: &str,
    channel: &str,
) -> Result<NotificationLog, sqlx::Error> {

    let row = sqlx::query(
        r#"
        INSERT INTO notification_logs (

            notification_event_id,

            recipient_email,

            channel,

            status

        )
        VALUES ($1,$2,$3,'pending')

        RETURNING
            id,
            notification_event_id,
            recipient_email,
            channel,
            status,
            error_message,
            created_at,
            sent_at
        "#
    )
    .bind(notification_event_id)
    .bind(recipient_email)
    .bind(channel)
    .fetch_one(pool)
    .await?;

    Ok(NotificationLog {

        id: row.get("id"),

        notification_event_id: row.get("notification_event_id"),

        recipient_email: row.get("recipient_email"),

        channel: row.get("channel"),

        status: row.get("status"),

        error_message: row.get("error_message"),

        created_at: row.get("created_at"),

        sent_at: row.get("sent_at"),
    })
}

pub async fn mark_sent(
    pool: &DbPool,
    id: i32,
) -> Result<(), sqlx::Error> {

    sqlx::query(
        r#"
        UPDATE notification_logs

        SET

            status = 'sent',

            sent_at = NOW(),

            error_message = NULL

        WHERE id = $1
        "#
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn mark_failed(
    pool: &DbPool,
    id: i32,
    error_message: &str,
) -> Result<(), sqlx::Error> {

    sqlx::query(
        r#"
        UPDATE notification_logs

        SET

            status = 'failed',

            error_message = $1

        WHERE id = $2
        "#
    )
    .bind(error_message)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}