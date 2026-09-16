use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::po_line_item::{
        CreatePoLineItemRequest,
        PoLineItem,
        UpdatePoLineItemRequest,
    },
    repositories::{po_item, po_line_item, purchase_order},
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