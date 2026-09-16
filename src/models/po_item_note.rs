use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PoItemNote {
    pub id: i32,

    pub po_item_id: i32,

    pub note: String,

    pub created_by: String,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePoItemNoteRequest {
    pub note: String,
}