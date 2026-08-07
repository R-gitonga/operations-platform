use serde::Serialize;

use chrono::{NaiveDate};

use crate::models::wso_item_detail::WsoItemDetail;

#[derive(Debug, Serialize)]
pub struct WsoDetail {

    pub id: i32,

    pub date_signed: Option<NaiveDate>,

    pub wso_number: String,

    pub req_number: Option<String>,

    pub attachment_name: Option<String>,

    pub attachment_path: Option<String>,

    pub status: String,

    pub total_items: usize,

    pub total_qty_raised: i32,

    pub total_qty_received: i32,

    pub total_balance: i32,

    pub items: Vec<WsoItemDetail>,
}