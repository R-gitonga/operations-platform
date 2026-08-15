use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PartialReceivingSettings {
    pub id: i32,
    pub attention_after_days: i32,
    pub updated_at: DateTime<Utc>,
}