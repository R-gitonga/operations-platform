use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::models::po_item_detail::PoItemDetail;

#[derive(Debug, Serialize)]
pub struct PurchaseOrderDetail {
    pub id: i32,

    pub erp_reference: String,

    pub accounts_reference: String,

    pub supplier_id: i32,

    pub supplier_name: String,

    pub description: Option<String>,

    pub attachment_path: Option<String>,

    pub attachment_name: Option<String>,

    pub status: String,

    pub total_items: usize,

    pub total_qty_ordered: i32,

    pub created_by: String,

    pub created_at: DateTime<Utc>,

    pub items: Vec<PoItemDetail>,
}