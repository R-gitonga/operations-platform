use crate::{
    database::DbPool,
    models::{
        notification_recipient::NotificationRecipient,
        notification_recipient::CreateNotificationRecipientRequest,
        notification_recipient::UpdateNotificationRecipientRequest,
    },
    repositories::notification_recipient,
};

pub async fn list(
    pool: &DbPool,
) -> Result<Vec<NotificationRecipient>, sqlx::Error> {

    notification_recipient::find_all(pool).await
}

pub async  fn create(
    pool: &DbPool,
    request: CreateNotificationRecipientRequest,
) -> Result<NotificationRecipient, sqlx::Error> {

    notification_recipient::create(
        pool,
        request,
    )
    .await
}

pub async fn update(
    pool: &DbPool,
    id: i32,
    request: UpdateNotificationRecipientRequest,
) -> Result<NotificationRecipient, sqlx::Error> {

    notification_recipient::update(
        pool,
        id,
        request,
    )
    .await
}

pub async fn find_enabled_by_event(
    pool: &DbPool,
    notification_event_id: i32,
) -> Result<Vec<NotificationRecipient>, sqlx::Error> {

    notification_recipient::find_enabled_by_event(
        pool, 
        notification_event_id
    )
    .await
}