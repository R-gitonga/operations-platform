use axum::{
    extract::{
        State,
        Path
    },
    Json,
};

use crate::{
    authenticated_user::AuthenticatedUser,
    app_state::AppState,
    errors::app_error::AppError,
    models::{
        notification_event::NotificationEvent,
        notification_setting::NotificationSetting,
        update_notification_setting::UpdateNotificationSetting,
    },
    services::settings,
};

pub async fn get_notification_events(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
) -> Result<Json<Vec<NotificationEvent>>, AppError> {

    let events =
        settings::get_notification_events(&state.pool).await?;

    Ok(Json(events))
}

pub async fn get_notification_settings(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
) -> Result<Json<Vec<NotificationSetting>>, AppError> {
    
    let settings =
        settings::get_notification_settings(&state.pool).await?;

        Ok(Json(settings))
}

pub async fn update_notification_setting(

    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(id): Path<i32>,

    Json(setting): Json<UpdateNotificationSetting>,

) -> Result<(), AppError> {

    settings::update_notification_setting(
        &state.pool,
        id,
        setting,
    )
    .await?;

    Ok(())
}
