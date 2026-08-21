use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WsoItemBrandingDetail {
    pub id: i32,
    pub wso_item_id: i32,

    pub branding_type_id: i32,
    pub branding_type_code: String,
    pub branding_type_name: String,

    pub branding_location_id: i32,
    pub branding_location_code: String,
    pub branding_location_name: String,

    pub quantity: i32,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}