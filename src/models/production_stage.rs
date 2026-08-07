use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductionStage {
    pub id: i32,

    pub code: String,

    pub display_name: String,

    pub display_order: i32,

    pub color: String,

    pub active: bool,

    pub expected_duration_hours: Option<i32>,

    pub attention_enabled: bool,
}