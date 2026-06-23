use serde::Serialize;

use crate::models::line_item::WsoLineItem;

#[derive(Debug, Serialize)]
pub struct WsoDetail {
    pub id: i32,
    pub wso_number: String,
    pub req_number: Option<String>,
    pub description: Option<String>,
    pub remarks: Option<String>,
    pub status: String,

    //Business statistics
    pub line_item_count: usize,
    pub total_quantity: i32,

    //child records
    pub line_items: Vec<WsoLineItem>,
}