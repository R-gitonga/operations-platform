use sqlx::{Postgres, Transaction};

use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::{
        po_item::{CreatePoItemRequest, PoItem, UpdatePoItemRequest},
        po_item_detail::PoItemDetail,
    },
    repositories::{po_item, po_item_note, po_line_item, purchase_order},
    services::po_rules,
};

fn validate_branding(
    branding_required: bool,
    branding_type_id: Option<i32>,
    branding_location_id: Option<i32>,
) -> Result<(), AppError> {

    if branding_required
        && (branding_type_id.is_none() || branding_location_id.is_none())
    {
        return Err(AppError::BadRequest(
            "Branding type and location are required when branding is marked as required."
                .to_string(),
        ));
    }

    Ok(())
}

fn validate_line_items(
    line_items: &[crate::models::po_line_item::CreatePoLineItemRequest],
) -> Result<(), AppError> {

    if line_items.is_empty() {
        return Err(AppError::BadRequest(
            "A PO item must have at least one size/quantity line.".to_string(),
        ));
    }

    for line in line_items {

        if line.size.trim().is_empty() {
            return Err(AppError::BadRequest(
                "Size cannot be empty.".to_string(),
            ));
        }

        if line.qty_ordered <= 0 {
            return Err(AppError::BadRequest(
                "Quantity ordered must be greater than zero.".to_string(),
            ));
        }
    }

    Ok(())
}

// Creates a po_item and all of its po_line_items inside the
// caller's transaction — used both when adding a single item to
// an existing PO and, via the same function, inside the PO's own
// nested create-with-items transaction.
pub async fn create_with_line_items_tx(
    tx: &mut Transaction<'_, Postgres>,
    purchase_order_id: i32,
    payload: &CreatePoItemRequest,
    created_by: &str,
) -> Result<PoItem, AppError> {

    validate_branding(
        payload.branding_required,
        payload.branding_type_id,
        payload.branding_location_id,
    )?;

    validate_line_items(&payload.line_items)?;

    let item =
        po_item::create_tx(tx, purchase_order_id, payload, created_by)
            .await?;

    for line in &payload.line_items {
        po_line_item::create_tx(tx, item.id, line).await?;
    }

    Ok(item)
}

pub async fn add_item(
    pool: &DbPool,
    purchase_order_id: i32,
    payload: &CreatePoItemRequest,
    created_by: &str,
) -> Result<PoItemDetail, AppError> {

    let order = purchase_order::find_by_id(pool, purchase_order_id)
        .await?
        .ok_or(AppError::NotFound)?;

    po_rules::ensure_can_edit(&order)?;

    let mut tx = pool.begin().await?;

    let item =
        create_with_line_items_tx(
            &mut tx,
            purchase_order_id,
            payload,
            created_by,
        )
        .await?;

    tx.commit().await?;

    get_item_detail(pool, item.id).await
}

pub async fn update_item(
    pool: &DbPool,
    id: i32,
    payload: &UpdatePoItemRequest,
) -> Result<PoItem, AppError> {

    validate_branding(
        payload.branding_required,
        payload.branding_type_id,
        payload.branding_location_id,
    )?;

    let item = po_item::find_by_id(pool, id)
        .await?
        .ok_or(AppError::NotFound)?;

    let order = purchase_order::find_by_id(pool, item.purchase_order_id)
        .await?
        .ok_or(AppError::NotFound)?;

    po_rules::ensure_can_edit(&order)?;

    Ok(
        po_item::update(
            pool,
            id,
            payload.category_id,
            payload.description.as_deref(),
            payload.expected_delivery_date,
            payload.branding_required,
            payload.branding_type_id,
            payload.branding_location_id,
        )
        .await?
    )
}

pub async fn get_item_detail(
    pool: &DbPool,
    id: i32,
) -> Result<PoItemDetail, AppError> {

    let item = po_item::find_by_id(pool, id)
        .await?
        .ok_or(AppError::NotFound)?;

    let line_items = po_line_item::find_by_item(pool, id).await?;

    let notes = po_item_note::find_by_item(pool, id).await?;

    let total_qty_ordered: i32 =
        line_items.iter().map(|l| l.qty_ordered).sum();

    Ok(PoItemDetail {
        id: item.id,
        purchase_order_id: item.purchase_order_id,
        category_id: item.category_id,
        description: item.description,
        expected_delivery_date: item.expected_delivery_date,
        branding_required: item.branding_required,
        branding_type_id: item.branding_type_id,
        branding_type_name: item.branding_type_name,
        branding_location_id: item.branding_location_id,
        branding_location_name: item.branding_location_name,
        total_qty_ordered,
        created_by: item.created_by,
        created_at: item.created_at,
        line_items,
        notes,
    })
}

// Used by the PO-header service to assemble a full
// PurchaseOrderDetail without needing to know how a single
// item's detail is built.
pub async fn get_items_detail_for_purchase_order(
    pool: &DbPool,
    purchase_order_id: i32,
) -> Result<Vec<PoItemDetail>, AppError> {

    let items = po_item::find_by_purchase_order(pool, purchase_order_id).await?;

    let mut details = Vec::with_capacity(items.len());

    for item in items {
        details.push(get_item_detail(pool, item.id).await?);
    }

    Ok(details)
}