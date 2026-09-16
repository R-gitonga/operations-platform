use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::po_item_note::{CreatePoItemNoteRequest, PoItemNote},
    repositories::{po_item, po_item_note},
};

pub async fn add_note(
    pool: &DbPool,
    po_item_id: i32,
    payload: &CreatePoItemNoteRequest,
    created_by: &str,
) -> Result<PoItemNote, AppError> {

    let note = payload.note.trim();

    if note.is_empty() {
        return Err(AppError::BadRequest(
            "Note cannot be empty.".to_string(),
        ));
    }

    // Confirm the item exists — deliberately NOT gated by the
    // PO's cancelled/active status. Notes are a commentary log,
    // not an operational action, so they should stay available
    // even on a cancelled PO for record-keeping.
    po_item::find_by_id(pool, po_item_id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(po_item_note::create(pool, po_item_id, note, created_by).await?)
}

pub async fn list_notes(
    pool: &DbPool,
    po_item_id: i32,
) -> Result<Vec<PoItemNote>, AppError> {
    Ok(po_item_note::find_by_item(pool, po_item_id).await?)
}