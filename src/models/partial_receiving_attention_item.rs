use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PartialReceivingAttentionItem {
    pub tracking_id: i32,
    pub wso_id: i32,
    pub wso_item_id: i32,
    pub wso_number: String,
    pub description: String,
    pub design_code: String,
    pub fabric_code: String,
    pub first_partial_received_at: DateTime<Utc>,
    pub attention_after_days: i32,
    pub elapsed_days: i64,
    pub overdue_days: i64,
    pub outstanding_quantity: i32,
}