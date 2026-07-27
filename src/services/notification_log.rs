use crate::{
    database::DbPool,
    models::notification_log::NotificationLog,
    repositories::notification_log,
};

pub async fn create_pending(
    pool: &DbPool,
    notification_event_id: i32,
    recipient_email: &str,
    channel: &str,
) -> Result<NotificationLog, sqlx::Error> {

    notification_log::create_pending(
        pool,
        notification_event_id,
        recipient_email,
        channel,
    )
    .await
}

pub async fn mark_sent(
    pool: &DbPool,
    id: i32,
) -> Result<(), sqlx::Error> {

    notification_log::mark_sent(
        pool,
        id,
    )
    .await
}

pub async fn mark_failed(
    pool: &DbPool,
    id: i32,
    error_message: &str,
) -> Result<(), sqlx::Error> {

    notification_log::mark_failed(
        pool,
        id,
        error_message,
    )
    .await
}