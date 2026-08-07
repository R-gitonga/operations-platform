use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    errors::app_error::AppError,
    models::{
        change_production_item_stage::ChangeProductionItemStageRequest,
        production_stage::ProductionStage,
        production_stage_item::ProductionStageItem,
        production_stage_requests::{
            CreateProductionStageRequest,
            UpdateProductionStageRequest,
        },
        wso_stage_history::WsoStageHistory,
    },
    services::{
        production_item_stage_history,
        production_stage,
    },
};

pub async fn list_production_stages(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProductionStage>>, AppError> {

    let stages =
        production_stage::list(&state.pool).await?;

    Ok(Json(stages))
}

pub async fn get_production_stage(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ProductionStage>, AppError> {

    let stage =
        production_stage::get(
            &state.pool,
            id,
        )
        .await?;

    Ok(Json(stage))
}

pub async fn create_production_stage(
    State(state): State<AppState>,
    Json(request): Json<CreateProductionStageRequest>,
) -> Result<Json<ProductionStage>, AppError> {

    let stage =
        production_stage::create(
            &state.pool,
            &request,
        )
        .await?;

    Ok(Json(stage))
}

pub async fn update_production_stage(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(request): Json<UpdateProductionStageRequest>,
) -> Result<Json<ProductionStage>, AppError> {

    let stage =
        production_stage::update(
            &state.pool,
            id,
            &request,
        )
        .await?;

    Ok(Json(stage))
}

pub async fn deactivate_production_stage(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ProductionStage>, AppError> {

    let stage =
        production_stage::deactivate(
            &state.pool,
            id,
        )
        .await?;

    Ok(Json(stage))
}

pub async fn get_stage_history(
    State(state): State<AppState>,
    Path(wso_item_id): Path<i32>,
) -> Result<Json<Vec<WsoStageHistory>>, AppError> {

    let history =
        production_item_stage_history::list(
            &state.pool,
            wso_item_id,
        )
        .await?;

    Ok(Json(history))
}

pub async fn change_stage(
    State(state): State<AppState>,
    Path(wso_item_id): Path<i32>,
    Json(request): Json<ChangeProductionItemStageRequest>,
) -> Result<(), AppError> {

    production_item_stage_history::create(
        &state.pool,
        wso_item_id,
        request,
    )
    .await?;

    Ok(())
}

pub async fn get_stage_items(
    State(state): State<AppState>,
    Path(stage_id): Path<i32>,
) -> Result<Json<Vec<ProductionStageItem>>, AppError> {

    let items =
        production_stage::get_stage_items(
            &state.pool,
            stage_id,
        )
        .await?;

    Ok(Json(items))
}