use axum::{
    extract::State,
    Json,
};

use crate::{
    app_state::AppState,
    authenticated_user::AuthenticatedUser,
    errors::app_error::AppError,
    models::{
        partial_receiving_settings::PartialReceivingSettings,
        update_partial_receiving_settings::UpdatePartialReceivingSettings,
    },
    services::partial_receiving_settings,
};

pub async fn get_settings(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
) -> Result<Json<PartialReceivingSettings>, AppError> {
    let settings =
        partial_receiving_settings::get_settings(
            &state.pool,
        )
        .await?;

    Ok(Json(settings))
}

pub async fn update_settings(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Json(payload): Json<UpdatePartialReceivingSettings>,
) -> Result<Json<PartialReceivingSettings>, AppError> {
    let settings =
        partial_receiving_settings::update_attention_after_days(
            &state.pool,
            payload.attention_after_days,
        )
        .await?;

    Ok(Json(settings))
}