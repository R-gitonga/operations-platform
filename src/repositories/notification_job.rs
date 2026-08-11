use crate::{
    database::DbPool,
    models::{
        enqueue_notification_job::EnqueueNotificationJob,
        notification_job::NotificationJob,
    },
};

pub async fn enqueue(
    pool: &DbPool,
    request: EnqueueNotificationJob,
) -> Result<NotificationJob, sqlx::Error> {

    sqlx::query_as::<_, NotificationJob>(
        r#"
        INSERT INTO notification_jobs (

            notification_log_id,

            sender_name,

            sender_email,

            recipient_email,

            subject,

            html_body

        )

        VALUES ($1,$2,$3,$4, $5, $6)

        RETURNING *
        "#
    )
    .bind(request.notification_log_id)
    .bind(request.sender_name)
    .bind(request.sender_email)
    .bind(request.recipient_email)
    .bind(request.subject)
    .bind(request.html_body)
    .fetch_one(pool)
    .await
}

pub async fn find_pending(
    pool: &DbPool,
) -> Result<Vec<NotificationJob>, sqlx::Error> {

    sqlx::query_as::<_, NotificationJob>(
        r#"
        SELECT *

        FROM notification_jobs

        WHERE status = 'pending'

        ORDER BY created_at
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn mark_sent(
    pool: &DbPool,
    id: i32,
) -> Result<(), sqlx::Error> {

    sqlx::query(
        r#"
        UPDATE notification_jobs

        SET

            status = 'sent',

            processed_at = NOW()

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
    error: &str,
) -> Result<(), sqlx::Error> {

    sqlx::query(
        r#"
        UPDATE notification_jobs

        SET

            status = 'failed',

            attempts = attempts + 1,

            error_message = $1,

            processed_at = NOW()

        WHERE id = $2
        "#
    )
    .bind(error)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}