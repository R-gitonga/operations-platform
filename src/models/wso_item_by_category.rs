use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct WsoItemByCategory {
    pub wso_id: i32,

    pub wso_number: String,

    pub wso_status: String,

    pub wso_item_id: i32,

    pub description: Option<String>,

    pub design_code: Option<String>,

    pub fabric_code: Option<String>,

    pub category_id: i32,

    pub category_name: String,

    pub current_stage_name: Option<String>,

    pub current_stage_color: Option<String>,

    pub total_qty_raised: i32,

    pub total_qty_received: i32,

    pub total_balance: i32,
}