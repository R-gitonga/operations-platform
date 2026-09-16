use std::collections::HashSet;

use crate::{
    config::Config,
    database::DbPool,
    errors::app_error::AppError,
    models::{
        po_receipt::{CreatePoReceiptRequest, PoReceipt, PoReceiptDetail},
        user::User,
    },
    repositories::{po_item, po_line_item, po_receipt, purchase_order},
    services::po_rules,
};

pub async fn record_receipt(
    pool: &DbPool,
    _config: &Config,
    purchase_order_id: i32,
    payload: CreatePoReceiptRequest,
    actor: &User,
) -> Result<PoReceiptDetail, AppError> {
    if payload.lines.is_empty() {
        return Err(AppError::BadRequest(
            "A receipt must include at least one line.".into(),
        ));
    }

    let order = purchase_order::find_by_id(pool, purchase_order_id)
        .await?
        .ok_or(AppError::NotFound)?;

    po_rules::ensure_can_receive(&order)?;

    let mut seen_line_items = HashSet::new();

    for line in &payload.lines {
        if line.qty_delivered <= 0 {
            return Err(AppError::BadRequest(
                "qty_delivered must be greater than zero.".into(),
            ));
        }

        if !seen_line_items.insert(line.po_line_item_id) {
            return Err(AppError::BadRequest(
                "Each line item can only appear once per receipt.".into(),
            ));
        }

        let line_item = po_line_item::find_by_id(pool, line.po_line_item_id)
            .await?
            .ok_or(AppError::NotFound)?;

        let item = po_item::find_by_id(pool, line_item.po_item_id)
            .await?
            .ok_or(AppError::NotFound)?;

        if item.purchase_order_id != purchase_order_id {
            return Err(AppError::BadRequest(
                "Line item does not belong to this Purchase Order.".into(),
            ));
        }

        let already_delivered =
            po_receipt::total_delivered_for_line(pool, line.po_line_item_id)
                .await?;

        if already_delivered + line.qty_delivered > line_item.qty_ordered {
            return Err(AppError::BadRequest(format!(
                "Delivered quantity exceeds quantity ordered for line item {}.",
                line.po_line_item_id
            )));
        }
    }

    let receipt = po_receipt::create(
        pool,
        purchase_order_id,
        &actor.name,
        &payload,
    )
    .await?;

    Ok(receipt)
}

pub async fn get_receipts_with_lines(
    pool: &DbPool,
    purchase_order_id: i32,
) -> Result<Vec<PoReceiptDetail>, AppError> {
    let receipts: Vec<PoReceipt> =
        po_receipt::find_by_purchase_order(pool, purchase_order_id).await?;

    let mut details = Vec::with_capacity(receipts.len());

    for receipt in receipts {
        let lines = po_receipt::find_lines_by_receipt(pool, receipt.id).await?;
        details.push(PoReceiptDetail { receipt, lines });
    }

    Ok(details)
}