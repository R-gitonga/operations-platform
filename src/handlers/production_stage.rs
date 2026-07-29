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
        wso_stage_history::WsoStageHistory,
    },
    services::{
        production_stage,
        production_item_stage_history,
    },
};

pub async fn list_production_stages(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProductionStage>>, AppError> {

    let stages =
        production_stage::list(&state.pool).await?;

    Ok(Json(stages))
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