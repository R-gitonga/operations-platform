use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::{
        create_purchase_order::CreatePurchaseOrderRequest,
        purchase_order::{PurchaseOrder, UpdatePurchaseOrderRequest},
        purchase_order_detail::PurchaseOrderDetail,
    },
    repositories::{purchase_order, supplier},
    services::{po_item, po_rules},
};

fn validate_references(
    erp_reference: &str,
    accounts_reference: &str,
) -> Result<(), AppError> {

    if erp_reference.trim().is_empty() {
        return Err(AppError::BadRequest(
            "ERP reference cannot be empty.".to_string(),
        ));
    }

    if accounts_reference.trim().is_empty() {
        return Err(AppError::BadRequest(
            "Accounts reference cannot be empty.".to_string(),
        ));
    }

    Ok(())
}

async fn ensure_supplier_selectable(
    pool: &DbPool,
    supplier_id: i32,
) -> Result<(), AppError> {

    let supplier = supplier::find_by_id(pool, supplier_id)
        .await?
        .ok_or(AppError::NotFound)?;

    if !supplier.active {
        return Err(AppError::BadRequest(
            "Cannot use an inactive supplier on a new Purchase Order."
                .to_string(),
        ));
    }

    Ok(())
}

pub async fn create_complete(
    pool: &DbPool,
    payload: &CreatePurchaseOrderRequest,
    created_by: &str,
) -> Result<PurchaseOrderDetail, AppError> {

    validate_references(&payload.erp_reference, &payload.accounts_reference)?;

    if payload.items.is_empty() {
        return Err(AppError::BadRequest(
            "A Purchase Order must contain at least one item.".to_string(),
        ));
    }

    ensure_supplier_selectable(pool, payload.supplier_id).await?;

    let mut tx = pool.begin().await?;

    let order = purchase_order::create_tx(
        &mut tx,
        payload.erp_reference.trim(),
        payload.accounts_reference.trim(),
        payload.supplier_id,
        payload.description.as_deref(),
        created_by,
    )
    .await?;

    for item_payload in &payload.items {
        po_item::create_with_line_items_tx(
            &mut tx,
            order.id,
            item_payload,
            created_by,
        )
        .await?;
    }

    tx.commit().await?;

    get_detail(pool, order.id).await
}

pub async fn find_all(
    pool: &DbPool,
) -> Result<Vec<PurchaseOrder>, AppError> {
    Ok(purchase_order::find_all(pool).await?)
}

pub async fn find_all_filtered(
    pool: &DbPool,
    search: Option<&str>,
    status: Option<&str>,
) -> Result<Vec<PurchaseOrder>, AppError> {
    Ok(purchase_order::find_all_filtered(pool, search, status).await?)
}

pub async fn get_detail(
    pool: &DbPool,
    id: i32,
) -> Result<PurchaseOrderDetail, AppError> {

    let order = purchase_order::find_by_id(pool, id)
        .await?
        .ok_or(AppError::NotFound)?;

    // let supplier = supplier::find_by_id(pool, order.supplier_id)
    //     .await?
    //     .ok_or(AppError::NotFound)?;

    let items = po_item::get_items_detail_for_purchase_order(pool, id).await?;

    let total_qty_ordered: i32 =
        items.iter().map(|i| i.total_qty_ordered).sum();

    Ok(PurchaseOrderDetail {
        id: order.id,
        erp_reference: order.erp_reference,
        accounts_reference: order.accounts_reference,
        supplier_id: order.supplier_id,
        supplier_name: order.supplier_name,
        description: order.description,
        attachment_path: order.attachment_path,
        attachment_name: order.attachment_name,
        status: order.status,
        total_items: items.len(),
        total_qty_ordered,
        created_by: order.created_by,
        created_at: order.created_at,
        items,
    })
}

pub async fn update(
    pool: &DbPool,
    id: i32,
    payload: &UpdatePurchaseOrderRequest,
) -> Result<PurchaseOrder, AppError> {

    let order = purchase_order::find_by_id(pool, id)
        .await?
        .ok_or(AppError::NotFound)?;

    po_rules::ensure_can_edit(&order)?;

    let erp_reference = payload
        .erp_reference
        .as_deref()
        .unwrap_or(&order.erp_reference);

    let accounts_reference = payload
        .accounts_reference
        .as_deref()
        .unwrap_or(&order.accounts_reference);

    validate_references(erp_reference, accounts_reference)?;

    let supplier_id = payload.supplier_id.unwrap_or(order.supplier_id);

    if supplier_id != order.supplier_id {
        ensure_supplier_selectable(pool, supplier_id).await?;
    }

    let description = payload
        .description
        .as_deref()
        .or(order.description.as_deref());

    Ok(
        purchase_order::update(
            pool,
            id,
            erp_reference,
            accounts_reference,
            supplier_id,
            description,
        )
        .await?
    )
}

pub async fn update_attachment(
    pool: &DbPool,
    id: i32,
    attachment_name: &str,
    attachment_path: &str,
) -> Result<PurchaseOrder, AppError> {

    purchase_order::find_by_id(pool, id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(
        purchase_order::update_attachment(pool, id, attachment_name, attachment_path)
            .await?
    )
}

pub async fn cancel(
    pool: &DbPool,
    id: i32,
) -> Result<PurchaseOrder, AppError> {

    let order = purchase_order::find_by_id(pool, id)
        .await?
        .ok_or(AppError::NotFound)?;

    po_rules::ensure_can_cancel(&order)?;

    Ok(purchase_order::cancel(pool, id).await?)
}

pub async fn reactivate(
    pool: &DbPool,
    id: i32,
) -> Result<PurchaseOrder, AppError> {

    let order = purchase_order::find_by_id(pool, id)
        .await?
        .ok_or(AppError::NotFound)?;

    po_rules::ensure_can_reactivate(&order)?;

    Ok(purchase_order::reactivate(pool, id).await?)
}