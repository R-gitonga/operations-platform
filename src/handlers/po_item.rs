use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    authenticated_user::AuthenticatedUser,
    errors::app_error::AppError,
    models::{
        po_item::{CreatePoItemRequest, PoItem, UpdatePoItemRequest},
        po_item_detail::PoItemDetail,
    },
    services::po_item,
};

pub async fn add_item(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(purchase_order_id): Path<i32>,
    Json(payload): Json<CreatePoItemRequest>,
) -> Result<Json<PoItemDetail>, AppError> {
    let created =
        po_item::add_item(
            &state.pool,
            purchase_order_id,
            &payload,
            &user.name,
        )
        .await?;

    Ok(Json(created))
}

pub async fn get_item(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(id): Path<i32>,
) -> Result<Json<PoItemDetail>, AppError> {
    let detail =
        po_item::get_item_detail(&state.pool, id).await?;

    Ok(Json(detail))
}

pub async fn update_item(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePoItemRequest>,
) -> Result<Json<PoItem>, AppError> {
    let updated =
        po_item::update_item(&state.pool, id, &payload).await?;

    Ok(Json(updated))
}