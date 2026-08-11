use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    authenticated_user::AuthenticatedUser,
    app_state::AppState,
    errors::app_error::AppError,
    models::line_item::{
        CreateWsoLineItemRequest,
        ReceiveLineItemRequest,
        UpdateWsoLineItemRequest,
        WsoLineItem,
    },
    services::line_item,
};

pub async fn create_line_item(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(wso_item_id): Path<i32>,
    Json(payload): Json<CreateWsoLineItemRequest>,
) -> Result<Json<WsoLineItem>, AppError> {
    let created = line_item::create(
        &state.pool,
        &state.config,
        wso_item_id,
        &payload,
        &user,
    )
    .await?;

    Ok(Json(created))
}

pub async fn get_line_items(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(wso_item_id): Path<i32>,
) -> Result<Json<Vec<WsoLineItem>>, AppError> {
    let items =
        line_item::find_by_wso(
            &state.pool,
            wso_item_id,
        )
        .await?;

    Ok(Json(items))
}

pub async fn get_line_item(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(line_item_id): Path<i32>,
) -> Result<Json<WsoLineItem>, AppError> {
    let item =
        line_item::find_by_id(
            &state.pool,
            line_item_id,
        )
        .await?;

    Ok(Json(item))
}

pub async fn update_line_item(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(line_item_id): Path<i32>,
    Json(payload): Json<UpdateWsoLineItemRequest>,
) -> Result<Json<WsoLineItem>, AppError> {
    let updated = line_item::update(
        &state.pool,
        &state.config,
        line_item_id,
        payload,
        &user,
    )
    .await?;

    Ok(Json(updated))
}

pub async fn receive_line_item(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<i32>,
    Json(payload): Json<ReceiveLineItemRequest>,
) -> Result<Json<WsoLineItem>, AppError> {
    let item = line_item::receive(
        &state.pool,
        &state.config,
        id,
        payload,
        &user,
    )
    .await?;

    Ok(Json(item))
}

pub async fn delete_line_item(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(line_item_id): Path<i32>,
) -> Result<Json<WsoLineItem>, AppError> {
    let deleted = line_item::delete(
        &state.pool,
        &state.config,
        line_item_id,
        &user,
    )
    .await?;

    Ok(Json(deleted))
}