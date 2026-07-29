use chrono::NaiveDateTime;
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
    pub current_stage_changed_at: Option<NaiveDateTime>,
    pub current_stage_notes: Option<String>,

    // Audit
    pub created_by: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}