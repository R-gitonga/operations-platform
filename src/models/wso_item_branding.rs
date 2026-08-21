use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WsoItemBranding {
    pub id: i32,
    pub wso_item_id: i32,
    pub branding_type_id: i32,
    pub branding_location_id: i32,
    pub quantity: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}