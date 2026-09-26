use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::{
        po_defect::{CreatePoDefectRequest, PoDefect, ResolvePoDefectRequest},
        user::User,
    },
    repositories::{po_defect, po_item, po_line_item, po_receipt, purchase_order},
    services::po_rules,
};

const VALID_RESOLUTION_TYPES: [&str; 4] =
    ["replaced", "returned_for_credit", "written_off", "accepted"];

pub async fn report_defect(
    pool: &DbPool,
    po_line_item_id: i32,
    payload: CreatePoDefectRequest,
    actor: &User,
) -> Result<PoDefect, AppError> {
    if payload.qty_defective <= 0 {
        return Err(AppError::BadRequest(
            "qty_defective must be greater than zero.".into(),
        ));
    }

    if payload.reason.trim().is_empty() {
        return Err(AppError::BadRequest(
            "A reason is required to report a defect.".into(),
        ));
    }

    let line_item = po_line_item::find_by_id(pool, po_line_item_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let item = po_item::find_by_id(pool, line_item.po_item_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let order = purchase_order::find_by_id(pool, item.purchase_order_id)
        .await?
        .ok_or(AppError::NotFound)?;

    // Reuses the same rule as receiving: no activity against a
    // cancelled Purchase Order.
    po_rules::ensure_can_receive(&order)?;

    let total_delivered =
        po_receipt::total_delivered_for_line(pool, po_line_item_id).await?;

    // Uses the non-accepted total (not the all-time total) so that
    // a defect resolved as 'accepted' frees up room to flag a new
    // issue against that same quantity later -- matching the same
    // math services::po_status uses.
    let currently_defective =
        po_defect::total_non_accepted_defective_for_line(pool, po_line_item_id)
            .await?;

    if currently_defective + payload.qty_defective > total_delivered {
        return Err(AppError::BadRequest(
            "Defective quantity exceeds quantity delivered for this line item.".into(),
        ));
    }

    let defect = po_defect::create(
        pool,
        po_line_item_id,
        payload.qty_defective,
        &payload.reason,
        &actor.name,
    )
    .await?;

    Ok(defect)
}

pub async fn resolve_defect(
    pool: &DbPool,
    defect_id: i32,
    payload: ResolvePoDefectRequest,
    actor: &User,
) -> Result<PoDefect, AppError> {
    if !VALID_RESOLUTION_TYPES.contains(&payload.resolution_type.as_str()) {
        return Err(AppError::BadRequest(format!(
            "Invalid resolution_type '{}'.",
            payload.resolution_type
        )));
    }

    let existing = po_defect::find_by_id(pool, defect_id)
        .await?
        .ok_or(AppError::NotFound)?;

    if existing.status.eq_ignore_ascii_case("resolved") {
        return Err(AppError::BadRequest(
            "This defect has already been resolved.".into(),
        ));
    }

    let resolved = po_defect::resolve(
        pool,
        defect_id,
        &payload.resolution_type,
        payload.resolution_notes.as_deref(),
        &actor.name,
    )
    .await?;

    Ok(resolved)
}

pub async fn get_by_line_item(
    pool: &DbPool,
    po_line_item_id: i32,
) -> Result<Vec<PoDefect>, AppError> {
    Ok(po_defect::find_by_line_item(pool, po_line_item_id).await?)
}