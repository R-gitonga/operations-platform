use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct WsoSummary {
    pub total_orders: i64,
    pub status_counts: HashMap<String, i64>,
    pub total_qty_raised: i64,
    pub total_qty_received: i64,
    pub total_balance: i64,
}
