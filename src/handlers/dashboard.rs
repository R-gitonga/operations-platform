use axum::{
    extract::State,
    Json,
};

use crate::{
    authenticated_user::AuthenticatedUser,
    app_state::AppState,
    errors::app_error::AppError,
    models::{
        attention_required_item::AttentionRequiredItem,
        dashboard::DashboardSummary,
    },
    services::{dashboard, production_stage},
};

pub async fn get_dashboard(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
) -> Result<Json<DashboardSummary>, AppError> {
    let summary =
        dashboard::get_dashboard(
            &state.pool,
        )
        .await?;

    Ok(Json(summary))
}

pub async fn get_attention_required(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
) -> Result<Json<Vec<AttentionRequiredItem>>, AppError> {
    let items =
        production_stage::get_attention_required_items(
            &state.pool,
            &state.config,
        )
        .await?;

    Ok(Json(items))
}