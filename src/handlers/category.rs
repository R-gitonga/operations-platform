use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    errors::app_error::AppError,
    models::category::{
        Category,
        CreateCategoryRequest,
        UpdateCategoryRequest,
    },
    services::category,
};

pub async fn get_categories(
    State(state): State<AppState>,
) -> Result<Json<Vec<Category>>, AppError> {
    let categories = category::find_all(&state.pool).await?;
    Ok(Json(categories))
}

pub async fn get_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Category>, AppError> {
    let category = category::find_by_id(&state.pool, id).await?;
    Ok(Json(category))
}

pub async fn create_category(
    State(state): State<AppState>,
    Json(payload): Json<CreateCategoryRequest>,
) -> Result<Json<Category>, AppError> {
    let category = category::create(&state.pool, &payload).await?;
    Ok(Json(category))
}

pub async fn update_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> Result<Json<Category>, AppError> {
    let category = category::update(&state.pool, id, &payload).await?;
    Ok(Json(category))
}

pub async fn delete_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Category>, AppError> {
    let category = category::delete(&state.pool, id).await?;
    Ok(Json(category))
}
