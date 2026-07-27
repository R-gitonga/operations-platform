use sqlx::Row;
use sqlx::postgres::PgRow;

use crate::{
    database::DbPool,
    models::notification_recipient::{
        CreateNotificationRecipientRequest,
        NotificationRecipient,
        UpdateNotificationRecipientRequest,
    },
};

const RECIPIENT_SELECT: &str = r#"
SELECT

    nr.id,

    nr.notification_event_id,

    ne.code,

    ne.display_name AS event_name,

    nr.display_name,

    nr.email,

    nr.enabled

FROM notification_recipients nr

JOIN notification_events ne
    ON ne.id = nr.notification_event_id
"#;

fn map_notification_recipient(
    row: PgRow,
) -> NotificationRecipient {
    NotificationRecipient {
        id: row.get("id"),

        notification_event_id: row.get("notification_event_id"),

        code: row.get("code"),

        event_name: row.get("event_name"),

        display_name: row.get("display_name"),

        email: row.get("email"),

        enabled: row.get("enabled"),
    }
}

pub async fn find_all(
    pool: &DbPool,
) -> Result<Vec<NotificationRecipient>, sqlx::Error> {
    let sql = format!(
        "{} ORDER BY ne.display_name, nr.display_name",
        RECIPIENT_SELECT
    );

    let recipients = sqlx::query(&sql)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(map_notification_recipient)
        .collect();

    Ok(recipients)
}

pub async fn create(
    pool: &DbPool,
    request: CreateNotificationRecipientRequest,
) -> Result<NotificationRecipient, sqlx::Error> {
    let inserted = sqlx::query(
        r#"
        INSERT INTO notification_recipients (

            notification_event_id,

            display_name,

            email,

            enabled

        )
        VALUES ($1,$2,$3,$4)

        RETURNING id
        "#,
    )
    .bind(request.notification_event_id)
    .bind(request.display_name)
    .bind(request.email)
    .bind(request.enabled)
    .fetch_one(pool)
    .await?;

    let id: i32 = inserted.get("id");

    let sql = format!(
        "{} WHERE nr.id = $1",
        RECIPIENT_SELECT
    );

    let row = sqlx::query(&sql)
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(map_notification_recipient(row))
}

pub async fn find_by_id(
    pool: &DbPool,
    id: i32,
) -> Result<NotificationRecipient, sqlx::Error> {
    let sql = format!(
        "{} WHERE nr.id = $1",
        RECIPIENT_SELECT
    );

    let row = sqlx::query(&sql)
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(map_notification_recipient(row))
}

pub async fn find_enabled_by_event(
    pool: &DbPool,
    notification_event_id: i32,
) -> Result<Vec<NotificationRecipient>, sqlx::Error> {
    let sql = format!(
        "{}
        WHERE nr.notification_event_id = $1
            AND nr.enabled = TRUE
        ORDER BY nr.display_name",
        RECIPIENT_SELECT
    );

    let recipients = sqlx::query(&sql)
        .bind(notification_event_id)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(map_notification_recipient)
        .collect();

    Ok(recipients)
}

pub async fn update(
    pool: &DbPool,
    id: i32,
    request: UpdateNotificationRecipientRequest,
) -> Result<NotificationRecipient, sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE notification_recipients

        SET

            display_name = COALESCE($1, display_name),

            email = COALESCE($2, email),

            enabled = COALESCE($3, enabled),

            updated_at = NOW()

        WHERE id = $4
        "#,
    )
    .bind(request.display_name)
    .bind(request.email)
    .bind(request.enabled)
    .bind(id)
    .execute(pool)
    .await?;

    find_by_id(pool, id).await
}