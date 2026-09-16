use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    authenticated_user::AuthenticatedUser,
    errors::app_error::AppError,
    models::po_item_note::{CreatePoItemNoteRequest, PoItemNote},
    services::po_item_note,
};

pub async fn add_note(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(po_item_id): Path<i32>,
    Json(payload): Json<CreatePoItemNoteRequest>,
) -> Result<Json<PoItemNote>, AppError> {
    let created =
        po_item_note::add_note(
            &state.pool,
            po_item_id,
            &payload,
            &user.name,
        )
        .await?;

    Ok(Json(created))
}

pub async fn get_notes(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(po_item_id): Path<i32>,
) -> Result<Json<Vec<PoItemNote>>, AppError> {
    let notes =
        po_item_note::list_notes(&state.pool, po_item_id).await?;

    Ok(Json(notes))
}