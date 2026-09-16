use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::po_receipt_line::{CreatePoReceiptLineRequest, PoReceiptLine};

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct PoReceipt {
    pub id: i32,
    pub purchase_order_id: i32,
    pub delivery_note_reference: Option<String>,
    pub notes: Option<String>,
    pub received_by: String,
    pub received_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PoReceiptDetail {
    #[serde(flatten)]
    pub receipt: PoReceipt,
    pub lines: Vec<PoReceiptLine>,
}

// received_by is deliberately NOT part of this payload -- it is
// stamped from the authenticated actor in the service layer, the
// same way WSO's receiving flow uses actor.name rather than a
// free-text field.
#[derive(Debug, Clone, Deserialize)]
pub struct CreatePoReceiptRequest {
    pub delivery_note_reference: Option<String>,
    pub notes: Option<String>,
    pub lines: Vec<CreatePoReceiptLineRequest>,
}