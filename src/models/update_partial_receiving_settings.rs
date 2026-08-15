use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdatePartialReceivingSettings {
    pub attention_after_days: i32,
}