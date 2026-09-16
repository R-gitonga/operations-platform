use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    authenticated_user::AuthenticatedUser,
    errors::app_error::AppError,
    models::po_receipt::{CreatePoReceiptRequest, PoReceiptDetail},
    services::po_receipt,
};

pub async fn create_receipt(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(purchase_order_id): Path<i32>,
    Json(payload): Json<CreatePoReceiptRequest>,
) -> Result<Json<PoReceiptDetail>, AppError> {
    let receipt = po_receipt::record_receipt(
        &state.pool,
        &state.config,
        purchase_order_id,
        payload,
        &user,
    )
    .await?;

    Ok(Json(receipt))
}

pub async fn get_receipts(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(purchase_order_id): Path<i32>,
) -> Result<Json<Vec<PoReceiptDetail>>, AppError> {
    let receipts =
        po_receipt::get_receipts_with_lines(&state.pool, purchase_order_id)
            .await?;

    Ok(Json(receipts))
}