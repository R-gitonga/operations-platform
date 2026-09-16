use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;

use crate::models::{
    po_item_note::PoItemNote,
    po_line_item::PoLineItem,
};

// Receiving/defect totals (total_qty_received, total_outstanding,
// total_defective) will be added here once po_receipts/po_defects
// exist — deliberately left out for now rather than stubbed with
// zeros, so the shape stays honest about what's actually derivable
// at this stage.
#[derive(Debug, Serialize)]
pub struct PoItemDetail {
    pub id: i32,

    pub purchase_order_id: i32,

    pub category_id: Option<i32>,

    pub description: Option<String>,

    pub expected_delivery_date: Option<NaiveDate>,

    pub branding_required: bool,

    pub branding_type_id: Option<i32>,

    pub branding_type_name: Option<String>,

    pub branding_location_id: Option<i32>,

    pub branding_location_name: Option<String>,

    pub total_qty_ordered: i32,

    pub created_by: String,

    pub created_at: DateTime<Utc>,

    pub line_items: Vec<PoLineItem>,

    pub notes: Vec<PoItemNote>,
}