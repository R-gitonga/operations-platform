use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
// use sqlx::FromRow;



#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWsoLineItemRequest {
    pub size: String,
    pub qty_raised: i32,
    pub qty_received: Option<i32>,
    pub received_date: Option<NaiveDate>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWsoLineItemRequest {
    pub size: Option<String>,
    pub qty_raised: Option<i32>,
    pub qty_received: Option<i32>,
    pub received_date: Option<NaiveDate>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct WsoLineItem {
    pub id: i32,
    pub wso_order_id: i32,
    pub size: String,
    pub qty_raised: i32,
    pub qty_received: i32,
    pub received_date: Option<NaiveDate>,
    pub status: String,
    pub balance: i32,
}
