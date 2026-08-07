use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct AttentionRequiredItem {
    pub wso_id: i32,
    pub wso_number: String,
    pub wso_item_id: i32,
    pub description: String,
    pub design_code: String,
    pub fabric_code: String,
    pub current_stage_id: i32,
    pub current_stage_name: String,
    pub current_stage_color: String,
    pub stage_started_at: DateTime<Utc>,
    pub expected_duration_hours: i32,
    pub elapsed_hours: f64,
    pub overdue_hours: f64,
}
