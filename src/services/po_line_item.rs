use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::{
        po_defect::PoDefect,
        po_line_item::{
            CreatePoLineItemRequest, PoLineItem, PoLineItemDetail,
            UpdatePoLineItemRequest,
        },
    },
    repositories::{po_defect, po_item, po_line_item, po_receipt, purchase_order},
    services::po_rules,
};

fn validate(size: &str, qty_ordered: i32) -> Result<(), AppError> {

    if size.trim().is_empty() {
        return Err(AppError::BadRequest(
            "Size cannot be empty.".to_string(),
        ));
    }

    if qty_ordered <= 0 {
        return Err(AppError::BadRequest(
            "Quantity ordered must be greater than zero.".to_string(),
        ));
    }

    Ok(())
}

// Loads the PO that owns a given po_item, so callers can apply
// the same edit-lock rules (e.g. cannot add/edit lines on a
// cancelled PO) without duplicating the lookup chain everywhere.
async fn load_owning_purchase_order(
    pool: &DbPool,
    po_item_id: i32,
) -> Result<crate::models::purchase_order::PurchaseOrder, AppError> {

    let item = po_item::find_by_id(pool, po_item_id)
        .await?
        .ok_or(AppError::NotFound)?;

    purchase_order::find_by_id(pool, item.purchase_order_id)
        .await?
        .ok_or(AppError::NotFound)
}

pub async fn add_line_item(
    pool: &DbPool,
    po_item_id: i32,
    payload: &CreatePoLineItemRequest,
) -> Result<PoLineItem, AppError> {

    validate(&payload.size, payload.qty_ordered)?;

    let order = load_owning_purchase_order(pool, po_item_id).await?;

    po_rules::ensure_can_edit(&order)?;

    Ok(po_line_item::create(pool, po_item_id, payload).await?)
}

pub async fn update_line_item(
    pool: &DbPool,
    id: i32,
    payload: &UpdatePoLineItemRequest,
) -> Result<PoLineItem, AppError> {

    validate(&payload.size, payload.qty_ordered)?;

    let line_item = po_line_item::find_by_id(pool, id)
        .await?
        .ok_or(AppError::NotFound)?;

    let order = load_owning_purchase_order(pool, line_item.po_item_id).await?;

    po_rules::ensure_can_edit(&order)?;

    Ok(
        po_line_item::update(pool, id, &payload.size, payload.qty_ordered)
            .await?
    )
}

async fn build_detail(
    pool: &DbPool,
    line_item: PoLineItem,
) -> Result<PoLineItemDetail, AppError> {

    let total_delivered =
        po_receipt::total_delivered_for_line(pool, line_item.id).await?;

    let total_defective =
        po_defect::total_non_accepted_defective_for_line(pool, line_item.id)
            .await?;

    let total_accepted = total_delivered - total_defective;
    let outstanding = line_item.qty_ordered - total_accepted;

    let defects: Vec<PoDefect> =
        po_defect::find_by_line_item(pool, line_item.id).await?;

    Ok(PoLineItemDetail {
        line_item,
        total_delivered,
        total_defective,
        total_accepted,
        outstanding,
        defects,
    })
}

pub async fn get_line_item_detail(
    pool: &DbPool,
    id: i32,
) -> Result<PoLineItemDetail, AppError> {

    let line_item = po_line_item::find_by_id(pool, id)
        .await?
        .ok_or(AppError::NotFound)?;

    build_detail(pool, line_item).await
}

// Used by the PO-item-header service to assemble every one of its
// lines' receiving/defect figures without needing to know how a
// single line's detail is built.
pub async fn get_line_items_detail_for_item(
    pool: &DbPool,
    po_item_id: i32,
) -> Result<Vec<PoLineItemDetail>, AppError> {

    let line_items = po_line_item::find_by_item(pool, po_item_id).await?;

    let mut details = Vec::with_capacity(line_items.len());

    for line_item in line_items {
        details.push(build_detail(pool, line_item).await?);
    }

    Ok(details)
}