use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RecentOrder {
    pub id: i32,
    pub wso_number: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct OutstandingOrder {
    pub id: i32,
    pub wso_number: String,
    pub outstanding_qty: i64,
}

#[derive(Debug, Serialize)]
pub struct DashboardSummary {
    pub total_orders: i64,

    pub active_orders: i64,
    pub partial_orders: i64,
    pub completed_orders: i64,
    pub cancelled_orders: i64,

    pub total_qty_raised: i64,
    pub total_qty_received: i64,
    pub total_balance: i64,

    pub recent_orders: Vec<RecentOrder>,
    pub largest_outstanding: Vec<OutstandingOrder>,
}