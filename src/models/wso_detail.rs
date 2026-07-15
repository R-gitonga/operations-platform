use serde::Serialize;

use crate::models::line_item::WsoLineItem;

#[derive(Debug, Serialize)]
pub struct WsoDetail {
    pub id: i32,
    pub category_id: Option<i32>,
    pub date_signed: Option<chrono::NaiveDate>,
    pub wso_number: String,
    pub req_number: Option<String>,
    pub description: Option<String>,
    pub design_code: Option<String>,
    pub fabric_code: Option<String>,
    pub remarks: Option<String>,
    pub attachment_name: Option<String>,
    pub attachment_path: Option<String>,
    pub status: String,

    //Business statistics
    pub line_item_count: usize,
    pub total_qty_raised: i32,
    pub total_qty_received: i32,
    pub total_balance: i32,

    //child records
    pub line_items: Vec<WsoLineItem>,
}
