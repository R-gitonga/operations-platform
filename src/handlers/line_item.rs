use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    errors::app_error::AppError,
    models::line_item::{
        CreateWsoLineItemRequest,
        UpdateWsoLineItemRequest,
        WsoLineItem,
    },
    repositories::line_item,
};

pub async fn create_line_item(
    State(state): State<AppState>,
    Path(wso_order_id): Path<i32>,
    Json(payload): Json<CreateWsoLineItemRequest>,
) -> Result<Json<WsoLineItem>, AppError> {
    let created = line_item::create(&state.pool, wso_order_id, &payload).await?;
    Ok(Json(created))
}

pub async fn get_line_items(
    State(state): State<AppState>,
    Path(wso_order_id): Path<i32>,
) -> Result<Json<Vec<WsoLineItem>>, AppError> {
    let items = line_item::find_by_wso(&state.pool, wso_order_id).await?;
    Ok(Json(items))
}

pub async fn get_line_item(
    State(state): State<AppState>,
    Path(line_item_id): Path<i32>,
) -> Result<Json<WsoLineItem>, AppError> {
    let item = line_item::find_by_id(&state.pool, line_item_id).await?;
    Ok(Json(item))
}

pub async fn update_line_item(
    State(state): State<AppState>,
    Path(line_item_id): Path<i32>,
    Json(payload): Json<UpdateWsoLineItemRequest>,
) -> Result<Json<WsoLineItem>, AppError> {
    let mut item = line_item::find_by_id(&state.pool, line_item_id).await?;

    if let Some(size) = payload.size {
        item.size = size;
    }
    if let Some(quantity) = payload.quantity {
        item.quantity = quantity;
    }

    let updated = line_item::update(&state.pool, &item).await?;
    Ok(Json(updated))
}

pub async fn delete_line_item(
    State(state): State<AppState>,
    Path(line_item_id): Path<i32>,
) -> Result<Json<WsoLineItem>, AppError> {
    let deleted = line_item::delete(&state.pool, line_item_id).await?;
    Ok(Json(deleted))
}
