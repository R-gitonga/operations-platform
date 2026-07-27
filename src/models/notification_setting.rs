use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct NotificationSetting {

    pub id: i32,

    pub notification_event_id: i32,

    pub code: String,

    pub display_name: String,

    pub description: Option<String>,

    pub enabled: bool,

    pub email_enabled: bool,

    pub in_app_enabled: bool,
}