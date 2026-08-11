use serde::Serialize;

use chrono::{DateTime, Utc};


#[derive(Debug, Serialize)]
pub struct OrderSummary {
    pub total: i64,
    pub active: i64,
    pub partial: i64,
    pub completed: i64,
    pub cancelled: i64,
}

#[derive(Debug, Serialize)]
pub struct ProductionSummary {
    pub qty_raised: i64,
    pub qty_received: i64,
    pub balance: i64,
}

#[derive(Debug, Serialize)]
pub struct ProductionStageSummary {
    pub stage_id: i32,
    pub stage_name: String,
    pub stage_color: String,
    pub item_count: i64,
}

#[derive(Debug, Serialize)]
pub struct ProductionStageItem {
    pub production_item_id: i32,
    pub wso_id: i32,
    pub wso_number: String,
    pub description: String,
    pub design_code: Option<String>,
    pub fabric_code: Option<String>,
    pub branding_required: bool,
    pub branding_completed: bool,
}

#[derive(Debug, Serialize)]
pub struct RecentActivity {
    pub changed_at: DateTime<Utc>,

    pub wso_id: i32,

    pub wso_number: String,

    pub wso_item_id: i32,

    pub description: String,

    pub stage_name: String,

    pub changed_by: Option<String>,

    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RecentActivityPage {
    pub items: Vec<RecentActivity>,
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
    pub total_pages: i64,
}

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
    pub orders: OrderSummary,

    pub production: ProductionSummary,

    pub production_stages: Vec<ProductionStageSummary>,

    pub recent_activity: RecentActivityPage,

    pub recent_orders: Vec<RecentOrder>,

    pub outstanding_orders: Vec<OutstandingOrder>,
}