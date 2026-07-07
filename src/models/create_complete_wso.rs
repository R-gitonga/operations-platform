use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

use crate::models::line_item::CreateWsoLineItemRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCompleteWsoRequest {
    pub category_id: Option<i32>,
    pub date_signed: Option<NaiveDate>,
    pub wso_number: String,
    pub req_number: Option<String>,
    pub description: Option<String>,
    pub design_code: Option<String>,
    pub fabric_code: Option<String>,
    pub remarks: Option<String>,
    pub line_items: Vec<CreateWsoLineItemRequest>,
}

