use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::po_line_item::CreatePoLineItemRequest;

// branding_type_name / branding_location_name are resolved via
// a join in the repository query, the same way wso_item resolves
// its current_stage_name — kept on the base row rather than a
// separate detail-only struct, since a PO item's branding intent
// is a scalar fact about the item, not a collection.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PoItem {
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

    pub created_by: String,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}

// Used both standalone (adding an item to an existing PO) and
// nested inside CreatePurchaseOrderRequest (creating a PO with
// its items in one transaction) — line_items is always required
// since a po_item with no sizes has nothing to receive against.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePoItemRequest {
    pub category_id: Option<i32>,
    pub description: Option<String>,
    pub expected_delivery_date: Option<NaiveDate>,
    pub branding_required: bool,
    pub branding_type_id: Option<i32>,
    pub branding_location_id: Option<i32>,
    pub line_items: Vec<CreatePoLineItemRequest>,
}

// Editing an item's own fields only — line items are managed
// through their own endpoints once the item exists.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePoItemRequest {
    pub category_id: Option<i32>,
    pub description: Option<String>,
    pub expected_delivery_date: Option<NaiveDate>,
    pub branding_required: bool,
    pub branding_type_id: Option<i32>,
    pub branding_location_id: Option<i32>,
}