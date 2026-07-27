use crate::{
    database::DbPool,
    models::notification_event::NotificationEvent,
};

pub async fn find_all(
    pool: &DbPool,
) -> Result<Vec<NotificationEvent>, sqlx::Error> {

    let events = sqlx::query_as::<_, NotificationEvent>(
        r#"
        SELECT
            id,
            code,
            display_name,
            description
        FROM notification_events
        ORDER BY display_name
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(events)
}