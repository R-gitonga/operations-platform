use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWsoItemBrandingRequest {
    pub branding_type_id: i32,
    pub branding_location_id: i32,
    pub quantity: i32,
}