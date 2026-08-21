use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WsoPartialReceiptEvent {
    pub id: i32,

    pub wso_item_id: i32,

    pub line_item_id: i32,

    pub quantity_received: i32,

    pub total_raised: i32,

    pub total_received: i32,

    pub balance: i32,

    pub received_by: String,

    pub received_at: DateTime<Utc>,
}