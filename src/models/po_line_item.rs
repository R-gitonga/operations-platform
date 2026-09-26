use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::po_defect::PoDefect;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PoLineItem {
    pub id: i32,

    pub po_item_id: i32,

    pub size: String,

    pub qty_ordered: i32,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreatePoLineItemRequest {
    pub size: String,
    pub qty_ordered: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePoLineItemRequest {
    pub size: String,
    pub qty_ordered: i32,
}

// total_accepted = total_delivered - total_defective (non-accepted
// defects only). outstanding = qty_ordered - total_accepted, and
// can rise back above zero after receiving is "done" if a defect
// is later reported against an already-delivered quantity.
#[derive(Debug, Clone, Serialize)]
pub struct PoLineItemDetail {
    #[serde(flatten)]
    pub line_item: PoLineItem,
    pub total_delivered: i32,
    pub total_defective: i32,
    pub total_accepted: i32,
    pub outstanding: i32,
    pub defects: Vec<PoDefect>,
}