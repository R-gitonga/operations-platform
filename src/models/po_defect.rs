use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct PoDefect {
    pub id: i32,
    pub po_line_item_id: i32,
    pub qty_defective: i32,
    pub reason: String,
    pub status: String,
    pub resolution_type: Option<String>,
    pub resolution_notes: Option<String>,
    pub reported_by: String,
    pub reported_at: DateTime<Utc>,
    pub resolved_by: Option<String>,
    pub resolved_at: Option<DateTime<Utc>>,
}

// po_line_item_id comes from the route path (/po-line-items/{id}/defects),
// the same convention as CreatePoLineItemRequest not repeating po_item_id.
#[derive(Debug, Clone, Deserialize)]
pub struct CreatePoDefectRequest {
    pub qty_defective: i32,
    pub reason: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResolvePoDefectRequest {
    pub resolution_type: String,
    pub resolution_notes: Option<String>,
}