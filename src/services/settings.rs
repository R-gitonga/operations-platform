use crate::{
    database::DbPool,
    models::{
        notification_event::NotificationEvent,
        notification_setting::NotificationSetting,
        update_notification_setting::UpdateNotificationSetting,
    },
    repositories::{
        notification_event,
        notification_setting,
    },
};

pub async fn get_notification_events(
    pool: &DbPool,
) -> Result<Vec<NotificationEvent>, sqlx::Error> {
    notification_event::find_all(pool).await
}

pub async fn get_notification_settings(
    pool: &DbPool,
) -> Result<Vec<NotificationSetting>, sqlx::Error> {

    notification_setting::find_all(pool).await
}

pub async fn update_notification_setting(
    pool: &DbPool,
    id: i32,
    setting: UpdateNotificationSetting,
) -> Result<(), sqlx::Error> {

    notification_setting::update(
        pool,
        id,
        &setting,
    )
    .await
}

pub async fn find_by_code(
    pool: &DbPool,
    code: &str,
) -> Result<NotificationSetting, sqlx::Error> {

    notification_setting::find_by_code(
        pool, 
        code
    )
    .await
}