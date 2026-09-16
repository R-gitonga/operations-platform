use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct PoSummary {
    pub total_orders: i64,
    pub status_counts: HashMap<String, i64>,
    pub total_qty_ordered: i64,
    pub total_qty_delivered: i64,
    pub total_qty_accepted: i64,
    pub total_qty_defective: i64,
    pub total_outstanding: i64,
}