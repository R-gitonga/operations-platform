use serde::{Deserialize, Serialize};

use crate::models::line_item::CreateWsoLineItemRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCompleteWsoRequest {
    pub wso_number: String,
    pub req_number: Option<String>,
    pub description: Option<String>,
    pub remarks: Option<String>,
    pub line_items: Vec<CreateWsoLineItemRequest>,
}

