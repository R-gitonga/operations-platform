use crate::{
    database::DbPool,
    models::{
        enqueue_notification_job::EnqueueNotificationJob,
        notification_job::NotificationJob,
    },
    repositories::notification_job,
};

pub async fn enqueue(
    pool: &DbPool,
    request: EnqueueNotificationJob,
) -> Result<NotificationJob, sqlx::Error> {

    notification_job::enqueue(
        pool,
        request,
    )
    .await
}

pub async fn find_pending(
    pool: &DbPool,
) -> Result<Vec<NotificationJob>, sqlx::Error> {

    notification_job::find_pending(pool).await
}

pub async fn mark_sent(
    pool: &DbPool,
    id: i32,
) -> Result<(), sqlx::Error> {

    notification_job::mark_sent(
        pool,
        id,
    )
    .await
}

pub async fn mark_failed(
    pool: &DbPool,
    id: i32,
    error: &str,
) -> Result<(), sqlx::Error> {

    notification_job::mark_failed(
        pool,
        id,
        error,
    )
    .await
}