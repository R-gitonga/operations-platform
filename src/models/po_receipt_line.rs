use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct PoReceiptLine {
    pub id: i32,
    pub po_receipt_id: i32,
     pub po_line_item_id: i32,
    pub qty_delivered: i32,
    pub total_delivered_to_date: i32,
    pub delivered_balance: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreatePoReceiptLineRequest {
    pub po_line_item_id: i32,
    pub qty_delivered: i32,
}