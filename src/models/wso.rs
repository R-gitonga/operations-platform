use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

/// data sent by client when creating a WSO

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWsoRequest {
    pub wso_number: String,
    pub req_number: Option<String>,
    pub description: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWsoRequest {
    pub wso_number: Option<String>,
    pub req_number: Option<String>,
    pub description: Option<String>,
    pub remarks: Option<String>,
    pub status: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct WsoOrder {
    pub id: i32,
    pub wso_number: String,
    pub req_number: Option<String>,
    pub description: Option<String>,
    pub remarks: Option<String>,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWsoLineItemRequest {
    pub size: String,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct WsoLineItem {
    pub id: i32,
    pub wso_order_id: i32,
    pub size: String,
    pub quantity: i32,
}