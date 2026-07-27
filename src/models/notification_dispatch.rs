use serde::Serialize;

use crate::models::notification_recipient::NotificationRecipient;

#[derive(Debug, Serialize)]
pub struct NotificationDispatch {
    pub event_code: String,

    pub event_name: String,

    pub email_enabled: bool,

    pub in_app_enabled: bool,

    pub recipients: Vec<NotificationRecipient>,
}