use axum::{extract::State, Json};

use crate::{
    app_state::AppState,
    authenticated_user::AuthenticatedUser,
    errors::app_error::AppError,
    models::{po_settings::PoSettings, update_po_settings::UpdatePoSettings},
    services::po_settings,
};

pub async fn get_settings(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
) -> Result<Json<PoSettings>, AppError> {
    let settings = po_settings::get_settings(&state.pool).await?;

    Ok(Json(settings))
}

pub async fn update_settings(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Json(payload): Json<UpdatePoSettings>,
) -> Result<Json<PoSettings>, AppError> {
    let settings =
        po_settings::update_settings(&state.pool, payload).await?;

    Ok(Json(settings))
}