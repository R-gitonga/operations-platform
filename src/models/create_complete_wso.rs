use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::models::line_item::CreateWsoLineItemRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBrandingRequirementRequest {
    pub branding_type_id: i32,
    pub branding_location_id: i32,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductionItemRequest {
    pub paid_order_reason: Option<String>,
    pub category_id: i32,

    pub description: String,

    pub design_code: String,

    pub fabric_code: String,

    pub branding_required: bool,

    pub branding: Vec<CreateBrandingRequirementRequest>,

    pub line_items: Vec<CreateWsoLineItemRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCompleteWsoRequest {
    #[serde(default)]
    pub is_paid_order: bool,
    pub branch_id: Option<i32>,
    pub date_signed: Option<NaiveDate>,

    pub wso_number: String,

    pub req_number: Option<String>,

    pub items: Vec<CreateProductionItemRequest>,
}
