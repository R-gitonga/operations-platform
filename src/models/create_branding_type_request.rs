use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBrandingTypeRequest {
    pub code: String,
    pub display_name: String,
    pub description: Option<String>,
    pub display_order: i32,
}