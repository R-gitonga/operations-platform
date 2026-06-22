use axum::{
    extract::{Path, State},
    Json,
};
// use serde_json::{json, Value};

use crate::{
    app_state::AppState,
    errors::app_error::AppError,
    models::wso::{CreateWsoRequest, UpdateWsoRequest, WsoOrder},
    repositories::wso,
};

pub async fn create_wso(
    State(state): State<AppState>,
    Json(payload): Json<CreateWsoRequest>,
) -> Result<Json<WsoOrder>, AppError> {
    let created = wso::create(&state.pool, &payload).await?;
    Ok(Json(created))
}

pub async fn get_wsos(
    State(state): State<AppState>,
) -> Result<Json<Vec<WsoOrder>>, AppError> {
    let wsos = wso::find_all(&state.pool).await?;
    Ok(Json(wsos))
}

pub async fn get_wso(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<WsoOrder>, AppError> {
    let wso_order = wso::find_by_id(&state.pool, id).await?;
    Ok(Json(wso_order))
}

pub async fn update_wso(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateWsoRequest>,
) -> Result<Json<WsoOrder>, AppError> {
    let mut wso_record = wso::find_by_id(&state.pool, id).await?;

    if let Some(val) = payload.wso_number {
        wso_record.wso_number = val;
    }
    if let Some(val) = payload.req_number {
        wso_record.req_number = Some(val);
    }
    if let Some(val) = payload.description {
        wso_record.description = Some(val);
    }
    if let Some(val) = payload.remarks {
        wso_record.remarks = Some(val);
    }
    if let Some(val) = payload.status {
        wso_record.status = val;
    }

    let updated = wso::update(&state.pool, &wso_record).await?;
    Ok(Json(updated))
}

pub async fn cancel_wso(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<WsoOrder>, AppError> {
    let cancelled = wso::cancel(&state.pool, id).await?;
    Ok(Json(cancelled))
}
