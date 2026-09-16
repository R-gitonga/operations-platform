use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSupplierRequest {
    pub name: String,
    pub contact_name: Option<String>,
    pub contact_info: Option<String>,
    pub active: bool,
}