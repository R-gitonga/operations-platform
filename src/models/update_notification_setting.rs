use serde::Deserialize;

//Request model not DB
#[derive(Debug, Deserialize)]
pub struct UpdateNotificationSetting {
    pub enabled: bool,

    pub email_enabled: bool,

    pub in_app_enabled: bool,
}