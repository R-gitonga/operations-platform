use axum::{
    extract::State,
    Json,
};

use crate::{
    app_state::AppState,
    errors::app_error::AppError,
    models::dashboard::DashboardSummary,
    services::dashboard,
};

pub async fn get_dashboard(
    State(state): State<AppState>,
) -> Result<Json<DashboardSummary>, AppError> {
    let summary = dashboard::get_dashboard(&state.pool).await?;

    Ok(Json(summary))
}