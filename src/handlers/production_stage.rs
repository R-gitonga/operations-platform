use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    errors::app_error::AppError,
    models::{
        change_wso_stage::ChangeWsoStageRequest,
        production_stage::ProductionStage,
        wso_stage_history::WsoStageHistory,
    },
    services::{
        production_stage,
        wso_stage_history,
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
    Path(wso_id): Path<i32>,
) -> Result<Json<Vec<WsoStageHistory>>, AppError> {

    let history =
        wso_stage_history::list(
            &state.pool,
            wso_id,
        )
        .await?;

    Ok(Json(history))
}

pub async fn change_stage(
    State(state): State<AppState>,
    Path(wso_id): Path<i32>,
    Json(request): Json<ChangeWsoStageRequest>,
) -> Result<(), AppError> {

    wso_stage_history::create(
        &state.pool,
        wso_id,
        request,
    )
    .await?;

    Ok(())
}