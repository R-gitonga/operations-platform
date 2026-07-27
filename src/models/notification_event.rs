use serde::Serialize;

use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct NotificationEvent {
    pub id: i32,
    pub code: String,
    pub display_name: String,
    pub description: Option<String>,
}

