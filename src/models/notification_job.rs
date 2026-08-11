use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NotificationJob {

    pub id: i32,

    pub notification_log_id: i32,

    pub sender_name: String,

    pub sender_email: String,

    pub recipient_email: String,

    pub subject: String,

    pub html_body: String,

    pub status: String,

    pub attempts: i32,

    pub error_message: Option<String>,

    pub created_at: DateTime<Utc>,

    pub processed_at: Option<DateTime<Utc>>,
}