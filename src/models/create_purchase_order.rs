use serde::{Deserialize, Serialize};

use crate::models::po_item::CreatePoItemRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePurchaseOrderRequest {
    pub erp_reference: String,

    pub accounts_reference: String,

    pub supplier_id: i32,

    pub description: Option<String>,

    pub items: Vec<CreatePoItemRequest>,
}