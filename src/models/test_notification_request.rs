use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TestNotificationRequest {
    pub event_code: String,

    pub wso_number: String,

    pub req_number: Option<String>,
}
