use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PoLineItem {
    pub id: i32,

    pub po_item_id: i32,

    pub size: String,

    pub qty_ordered: i32,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePoLineItemRequest {
    pub size: String,
    pub qty_ordered: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePoLineItemRequest {
    pub size: String,
    pub qty_ordered: i32,
}