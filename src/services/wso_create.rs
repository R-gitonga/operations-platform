use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::{
        create_complete_wso::CreateCompleteWsoRequest,
        wso_detail::WsoDetail,
    },
    repositories::{
        line_item,
        wso,
    },
};

use super::line_item as line_item_service;

pub async fn create_complete_wso(
    pool: &DbPool,
    payload: &CreateCompleteWsoRequest,
) -> Result<WsoDetail, AppError> {
    for item_payload in &payload.line_items {
        line_item_service::validate_create_payload(item_payload)?;
    }

    let mut tx = pool.begin().await?;
    let created_wso = wso::create_tx(&mut tx, payload).await?;

    let mut created_line_items = Vec::with_capacity(payload.line_items.len());
    for item_payload in &payload.line_items {
        let created_item = line_item::create_tx(&mut tx, created_wso.id, item_payload).await?;
        created_line_items.push(created_item);
    }

    let total_qty_raised: i32 = created_line_items.iter().map(|item| item.qty_raised).sum();
    let total_qty_received: i32 = created_line_items.iter().map(|item| item.qty_received).sum();
    let total_balance: i32 = created_line_items.iter().map(|item| item.balance).sum();
    let line_item_count = created_line_items.len();

    let wso_detail = WsoDetail {
        id: created_wso.id,
        category_id: created_wso.category_id,
        date_signed: created_wso.date_signed,
        wso_number: created_wso.wso_number,
        req_number: created_wso.req_number,
        description: created_wso.description,
        design_code: created_wso.design_code,
        fabric_code: created_wso.fabric_code,
        remarks: created_wso.remarks,
        attachment_name: created_wso.attachment_name,
        attachment_path: created_wso.attachment_path,
        status: created_wso.status,
        line_item_count,
        total_qty_raised,
        total_qty_received,
        total_balance,
        line_items: created_line_items,
    };

    tx.commit().await?;
    Ok(wso_detail)
}
