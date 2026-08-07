use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductionStageRequest {

    pub code: String,

    pub display_name: String,

    pub display_order: i32,

    pub color: String,

    pub expected_duration_hours: Option<i32>,

    pub attention_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProductionStageRequest {

    pub code: String,

    pub display_name: String,

    pub display_order: i32,

    pub color: String,

    pub expected_duration_hours: Option<i32>,

    pub attention_enabled: bool,
}