use serde::Serialize;

use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ProductionStageItem {
    pub wso_id: i32,
    pub wso_number: String,
    pub wso_item_id: i32,
    pub description: String,
    pub design_code: String,
    pub fabric_code: String,
    pub stage_name: String,
    pub stage_color: String,
    pub current_stage_changed_at: Option<DateTime<Utc>>,
    pub current_stage_changed_by: Option<String>,
}