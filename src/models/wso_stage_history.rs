use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WsoStageHistory {
    pub id: i32,


    pub wso_item_id: i32,

    pub production_stage_id: i32,

    pub stage_name: String,

    pub notes: Option<String>,

    pub changed_by: String,

    pub changed_at: NaiveDateTime,
}