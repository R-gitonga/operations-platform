use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWsoRequest {
    pub category_id: Option<i32>,
    pub date_signed: Option<NaiveDate>,
    pub wso_number: Option<String>,
    pub req_number: Option<String>,
    pub description: Option<String>,
    pub design_code: Option<String>,
    pub fabric_code: Option<String>,
    pub remarks: Option<String>,
    pub attachment_name: Option<String>,
    pub attachment_path: Option<String>,
    pub status: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct WsoOrder {
    pub id: i32,
    pub category_id: Option<i32>,
    pub date_signed: Option<NaiveDate>,
    pub wso_number: String,
    pub req_number: Option<String>,
    pub description: Option<String>,
    pub design_code: Option<String>,
    pub fabric_code: Option<String>,
    pub remarks: Option<String>,
    pub attachment_name: Option<String>,
    pub attachment_path: Option<String>,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

