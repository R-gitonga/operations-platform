use serde::{Deserialize, Serialize};
// use sqlx::FromRow;



#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWsoLineItemRequest {
    pub size: String,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWsoLineItemRequest {
    pub size: Option<String>,
    pub quantity: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct WsoLineItem {
    pub id: i32,
    pub wso_order_id: i32,
    pub size: String,
    pub quantity: i32,
}