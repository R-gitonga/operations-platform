use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PoOrderSummary {
    pub total: i64,
    pub active: i64,
    pub partial: i64,
    pub completed: i64,
    pub cancelled: i64,
}

#[derive(Debug, Serialize)]
pub struct PoQuantitySummary {
    pub qty_ordered: i64,
    pub qty_delivered: i64,
    pub qty_accepted: i64,
    pub qty_defective: i64,
    pub outstanding: i64,
}

#[derive(Debug, Serialize)]
pub struct PoSupplierSummary {
    pub supplier_id: i32,
    pub supplier_name: String,
    // Non-cancelled orders for this supplier -- active or partial.
    pub open_orders: i64,
    pub total_qty_ordered: i64,
}

#[derive(Debug, Serialize)]
pub struct PoRecentOrder {
    pub id: i32,
    pub accounts_reference: String,
    pub supplier_name: String,
    pub derived_status: String,
}

#[derive(Debug, Serialize)]
pub struct PoOutstandingOrder {
    pub id: i32,
    pub accounts_reference: String,
    pub supplier_name: String,
    pub outstanding_qty: i64,
}

// Merged, paginated feed across every PO -- same event sources as
// the per-item Procurement Timeline (notes/receipts/defects), just
// unfiltered by po_item_id and joined up to the owning PO.
#[derive(Debug, Serialize)]
pub struct PoRecentActivity {
    pub changed_at: DateTime<Utc>,
    pub purchase_order_id: i32,
    pub accounts_reference: String,
    pub po_item_id: i32,
    pub item_description: Option<String>,
    // "note" | "receipt" | "defect"
    pub event_type: String,
    pub size: Option<String>,
    pub changed_by: String,
    pub note: Option<String>,
    pub qty_delivered: Option<i32>,
    pub total_delivered_to_date: Option<i32>,
    pub delivered_balance: Option<i32>,
    pub qty_defective: Option<i32>,
    pub reason: Option<String>,
    pub defect_status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PoRecentActivityPage {
    pub items: Vec<PoRecentActivity>,
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize)]
pub struct PoDashboardSummary {
    pub orders: PoOrderSummary,
    pub quantities: PoQuantitySummary,
    pub by_supplier: Vec<PoSupplierSummary>,
    pub recent_orders: Vec<PoRecentOrder>,
    pub largest_outstanding: Vec<PoOutstandingOrder>,
    pub recent_activity: PoRecentActivityPage,
}

// outstanding and days_overdue are typed i64 deliberately -- both
// come from arithmetic over a SUM()'d subquery result (BIGINT),
// the same class of type that bit us with the NUMERIC/INT8
// mismatch earlier. Explicit ::BIGINT casts in the query back this
// up rather than relying on implicit promotion.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PoOverdueItem {
    pub purchase_order_id: i32,
    pub accounts_reference: String,
    pub supplier_name: String,
    pub po_item_id: i32,
    pub description: Option<String>,
    pub size: String,
    pub expected_delivery_date: NaiveDate,
    pub days_overdue: i64,
    pub outstanding: i64,
}