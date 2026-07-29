use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWsoItemRequest {
    pub category_id: Option<i32>,
    pub description: Option<String>,
    pub design_code: Option<String>,
    pub fabric_code: Option<String>,
    pub branding_required: bool,
}