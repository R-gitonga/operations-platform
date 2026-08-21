use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BrandingType {
    pub id: i32,

    pub code: String,

    pub display_name: String,

    pub description: Option<String>,

    pub display_order: i32,

    pub active: bool,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBrandingTypeRequest {
    pub code: String,

    pub display_name: String,

    pub description: Option<String>,

    pub display_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBrandingTypeRequest {
    pub code: String,

    pub display_name: String,

    pub description: Option<String>,

    pub display_order: i32,

    pub active: bool,
}