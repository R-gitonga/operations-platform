use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TestNotificationRequest {
    pub event_code: String,

    pub actor_name: String,

    pub actor_email: String,

    pub wso_number: String,

    pub department: String,

    pub description: String,
}