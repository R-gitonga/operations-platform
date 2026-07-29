use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWsoRequest {
    pub date_signed: Option<NaiveDate>,
    pub wso_number: Option<String>,
    pub req_number: Option<String>,
    pub attachment_name: Option<String>,
    pub attachment_path: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WsoOrder {

    pub id: i32,

    pub date_signed: Option<NaiveDate>,

    pub wso_number: String,

    pub req_number: Option<String>,

    pub attachment_name: Option<String>,

    pub attachment_path: Option<String>,

    pub status: String,

    pub created_at: Option<DateTime<Utc>>,

    pub updated_at: Option<DateTime<Utc>>,
}