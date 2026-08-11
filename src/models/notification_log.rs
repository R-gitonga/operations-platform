use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NotificationLog {
    pub id: i32,

    pub notification_event_id: i32,

    pub recipient_email: String,

    pub channel: String,

    pub status: String,

    pub error_message: Option<String>,

    pub created_at: DateTime<Utc>,

    pub sent_at: Option<DateTime<Utc>>,
}