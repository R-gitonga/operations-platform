use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct NotificationRecipient {
    pub id: i32,

    pub notification_event_id: i32,

    pub code: String,

    pub event_name: String,

    pub display_name: String,

    pub email: String,

    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateNotificationRecipientRequest {
    pub notification_event_id: i32,

    pub display_name: String,

    pub email: String,

    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateNotificationRecipientRequest {
    pub display_name: Option<String>,

    pub email: Option<String>,

    pub enabled: Option<bool>,
}