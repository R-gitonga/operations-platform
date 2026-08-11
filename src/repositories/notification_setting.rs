use crate::{
    database::DbPool,
    models::{
        notification_setting::NotificationSetting,
        update_notification_setting::UpdateNotificationSetting,
    }
};

pub async fn find_all(
    pool: &DbPool,
) -> Result<Vec<NotificationSetting>, sqlx::Error> {

    let settings = sqlx::query_as::<_, NotificationSetting>(
        r#"
        SELECT
            ns.id,
            ns.notification_event_id,
            ne.code,
            ne.display_name,
            ne.description,
            ns.enabled,
            ns.email_enabled,
            ns.in_app_enabled
        FROM notification_settings ns
        JOIN notification_events ne
            ON ne.id = ns.notification_event_id
        ORDER BY ne.display_name
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(settings)
}

pub async fn update(
    pool: &DbPool,
    id: i32,
    setting: &UpdateNotificationSetting,
) -> Result<(), sqlx::Error> {

    sqlx::query(
    r#"
        UPDATE notification_settings

        SET
            enabled = $1,
            email_enabled = $2,
            in_app_enabled = $3,

            updated_at = NOW()
        WHERE id = $4
        "#,
    )
    .bind(setting.enabled)
    .bind(setting.email_enabled)
    .bind(setting.in_app_enabled)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn find_by_code(
    pool: &DbPool,
    code: &str,
) -> Result<NotificationSetting, sqlx::Error> {
    let setting = sqlx::query_as::<_, NotificationSetting>(
        r#"
        SELECT
            ns.id,
            ns.notification_event_id,
            ne.code,
            ne.display_name,
            ne.description,
            ns.enabled,
            ns.email_enabled,
            ns.in_app_enabled
        FROM notification_settings ns
        JOIN notification_events ne
            ON ne.id = ns.notification_event_id
        WHERE ne.code =$1
        "#
    )
    .bind(code)
    .fetch_one(pool)
    .await?;

    Ok(setting)
}