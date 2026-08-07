use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WsoItem {

    pub id: i32,

    pub wso_order_id: i32,

    pub category_id: Option<i32>,

    pub description: Option<String>,

    pub design_code: Option<String>,

    pub fabric_code: Option<String>,

    pub branding_required: bool,

    pub branding_completed: bool,

    // Current production state
    pub current_stage_id: Option<i32>,
    pub current_stage_name: Option<String>,
    pub current_stage_color: Option<String>,
    pub current_stage_changed_by: Option<String>,
    pub current_stage_changed_at: Option<DateTime<Utc>>,
    pub current_stage_notes: Option<String>,
    pub status: String,

    // Audit
    pub created_by: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}