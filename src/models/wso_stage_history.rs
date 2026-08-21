use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WsoStageHistory {
    // Prefixed ("stage-7" / "receipt-12") since entries come
    // from two different source tables (wso_stage_history and
    // wso_partial_receipt_events) and their numeric ids can
    // collide. Read-only — only used as a React list key.
    pub id: String,

    pub wso_item_id: i32,

    // "stage_change" | "partial_received"
    pub event_type: String,

    // The production stage this row represents, when
    // event_type is "stage_change". NULL for receiving
    // events, since they aren't tied to a single stage.
    pub production_stage_id: Option<i32>,

    // Display label for the timeline entry: the stage's
    // display_name for a stage change, or a fixed label
    // ("Partially Received") for a receiving event.
    pub stage_name: String,

    // Colour used to render the timeline entry's border/badge.
    // Comes from production_stages.color for a stage change,
    // or a fixed colour for a receiving event.
    pub stage_color: String,

    pub notes: Option<String>,

    // Only populated for event_type = "partial_received".
    pub quantity_received: Option<i32>,

    pub total_raised: Option<i32>,

    pub balance: Option<i32>,

    pub changed_by: String,

    pub changed_at: DateTime<Utc>,
}