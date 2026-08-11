use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnqueueNotificationJob {

    pub notification_log_id: i32,

    pub sender_name: String,

    pub sender_email: String,

    pub recipient_email: String,

    pub subject: String,

    pub html_body: String,
}