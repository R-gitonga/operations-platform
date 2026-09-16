use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    authenticated_user::AuthenticatedUser,
    errors::app_error::AppError,
    models::po_defect::{CreatePoDefectRequest, PoDefect, ResolvePoDefectRequest},
    services::po_defect,
};

pub async fn report_defect(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(po_line_item_id): Path<i32>,
    Json(payload): Json<CreatePoDefectRequest>,
) -> Result<Json<PoDefect>, AppError> {
    let defect = po_defect::report_defect(
        &state.pool,
        po_line_item_id,
        payload,
        &user,
    )
    .await?;

    Ok(Json(defect))
}

pub async fn get_defects(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(po_line_item_id): Path<i32>,
) -> Result<Json<Vec<PoDefect>>, AppError> {
    let defects =
        po_defect::get_by_line_item(&state.pool, po_line_item_id).await?;

    Ok(Json(defects))
}

pub async fn resolve_defect(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<i32>,
    Json(payload): Json<ResolvePoDefectRequest>,
) -> Result<Json<PoDefect>, AppError> {
    let resolved =
        po_defect::resolve_defect(&state.pool, id, payload, &user).await?;

    Ok(Json(resolved))
}