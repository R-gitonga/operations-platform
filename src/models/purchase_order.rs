use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PurchaseOrder {
    pub id: i32,

    pub erp_reference: String,

    pub accounts_reference: String,

    pub supplier_id: i32,
    
    pub supplier_name: String,

    pub description: Option<String>,

    pub attachment_path: Option<String>,

    pub attachment_name: Option<String>,

    pub status: String,

    pub created_by: String,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}

// Partial update — only the editable header fields. Attachments
// are handled by a dedicated upload endpoint (mirrors WSO), and
// status only ever changes via the dedicated cancel/reactivate
// actions, not through this request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePurchaseOrderRequest {
    pub erp_reference: Option<String>,
    pub accounts_reference: Option<String>,
    pub supplier_id: Option<i32>,
    pub supplier_name: Option<String>,
    pub description: Option<String>,
}