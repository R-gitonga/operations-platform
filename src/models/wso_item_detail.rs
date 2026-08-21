// src/models/wso_item_detail.rs

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::{
    models::{
        line_item::WsoLineItem,
        wso_item_branding_detail::WsoItemBrandingDetail,
    },
};

#[derive(Debug, Serialize)]
pub struct WsoItemDetail {
    pub id: i32,

    pub category_id: Option<i32>,

    pub description: Option<String>,

    pub design_code: Option<String>,

    pub fabric_code: Option<String>,

    pub branding_required: bool,

    pub branding_completed: bool,

    pub status: String,

    pub current_stage_id: Option<i32>,

    pub current_stage_name: Option<String>,

    pub current_stage_color: Option<String>,

    pub current_stage_changed_by: Option<String>,

    pub current_stage_changed_at: Option<DateTime<Utc>>,

    pub current_stage_notes: Option<String>,

    pub total_qty_raised: i32,

    pub total_qty_received: i32,

    pub total_balance: i32,

    pub line_items: Vec<WsoLineItem>,

    pub branding: Vec<WsoItemBrandingDetail>,
}