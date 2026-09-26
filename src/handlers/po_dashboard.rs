use axum::{
    extract::{Query, State},
    Json,
};

use serde::Deserialize;

use crate::{
    app_state::AppState,
    authenticated_user::AuthenticatedUser,
    errors::app_error::AppError,
    models::po_dashboard::{PoDashboardSummary, PoOverdueItem},
    services::{po_dashboard, po_overdue},
};

#[derive(Debug, Deserialize)]
pub struct PoDashboardQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

pub async fn get_dashboard(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Query(query): Query<PoDashboardQuery>,
) -> Result<Json<PoDashboardSummary>, AppError> {

    let page = query.page.unwrap_or(1).max(1);

    let page_size = query.page_size.unwrap_or(10).clamp(1, 100);

    let summary =
        po_dashboard::get_dashboard(&state.pool, page, page_size).await?;

    Ok(Json(summary))
}

pub async fn get_overdue(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
) -> Result<Json<Vec<PoOverdueItem>>, AppError> {

    let items = po_overdue::get_overdue_items(&state.pool).await?;

    Ok(Json(items))
}