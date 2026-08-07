use chrono::{NaiveDate};

use serde::{Deserialize, Serialize};

use crate::models::line_item::CreateWsoLineItemRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductionItemRequest {
    pub category_id: i32,

    pub description: String,

    pub design_code: String,

    pub fabric_code: String,

    pub branding_required: bool,

    pub branding_completed: bool,

    pub line_items: Vec<CreateWsoLineItemRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCompleteWsoRequest {
    pub date_signed: Option<NaiveDate>,

    pub wso_number: String,

    pub req_number: Option<String>,

    pub items: Vec<CreateProductionItemRequest>,
}