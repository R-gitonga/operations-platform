use axum::{
    extract::{Query, State},
    Json,
};

use serde::Deserialize;

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

#[derive(Debug, Deserialize)]
pub struct DashboardQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

pub async fn get_dashboard(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Query(query): Query<DashboardQuery>,
) -> Result<Json<DashboardSummary>, AppError> {

    let page = query.page.unwrap_or(1).max(1);

    let page_size = query
        .page_size
        .unwrap_or(10)
        .clamp(1, 100);

    let summary =
        dashboard::get_dashboard(
            &state.pool,
            page,
            page_size,
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