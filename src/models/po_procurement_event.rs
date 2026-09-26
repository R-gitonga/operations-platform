use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Prefixed ("note-3" / "receipt-12" / "defect-5") since entries
// come from three different source tables and their numeric ids
// can collide. Read-only -- only used as a React list key.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PoProcurementEvent {
    pub id: String,

    pub po_item_id: i32,

    // "note" | "receipt" | "defect"
    pub event_type: String,

    // The line item's size, for "receipt"/"defect" events. NULL
    // for "note" events, which aren't tied to a single line.
    pub size: Option<String>,

    pub note: Option<String>,

    // Only populated for event_type = "receipt".
    pub qty_delivered: Option<i32>,
    pub total_delivered_to_date: Option<i32>,
    pub delivered_balance: Option<i32>,
    pub delivery_note_reference: Option<String>,

    // Only populated for event_type = "defect". A resolved defect
    // still renders as a single timeline entry (at reported_at)
    // with its resolution shown inline, rather than a second
    // entry at resolved_at -- kept simple, same economy WSO's
    // stage-history view uses (one row per record).
    pub qty_defective: Option<i32>,
    pub reason: Option<String>,
    pub defect_status: Option<String>,
    pub resolution_type: Option<String>,
    pub resolution_notes: Option<String>,

    pub changed_by: String,

    pub changed_at: DateTime<Utc>,
}