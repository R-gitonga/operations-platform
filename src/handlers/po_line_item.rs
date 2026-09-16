use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    authenticated_user::AuthenticatedUser,
    errors::app_error::AppError,
    models::po_line_item::{
        CreatePoLineItemRequest,
        PoLineItem,
        UpdatePoLineItemRequest,
    },
    services::po_line_item,
};

pub async fn add_line_item(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(po_item_id): Path<i32>,
    Json(payload): Json<CreatePoLineItemRequest>,
) -> Result<Json<PoLineItem>, AppError> {
    let created =
        po_line_item::add_line_item(&state.pool, po_item_id, &payload)
            .await?;

    Ok(Json(created))
}

pub async fn update_line_item(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePoLineItemRequest>,
) -> Result<Json<PoLineItem>, AppError> {
    let updated =
        po_line_item::update_line_item(&state.pool, id, &payload)
            .await?;

    Ok(Json(updated))
}