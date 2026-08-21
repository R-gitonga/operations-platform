use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBrandingLocationRequest {
    pub code: String,
    pub display_name: String,
    pub description: Option<String>,
    pub display_order: i32,
    pub active: bool,
}