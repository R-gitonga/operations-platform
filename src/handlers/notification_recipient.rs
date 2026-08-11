use axum::{
    extract::{
        State,
        Path,
    },
    Json,
};

use crate::{
    authenticated_user::AuthenticatedUser,
    app_state::AppState,
    errors::app_error::AppError,
    models::{
        notification_recipient::NotificationRecipient,
        notification_recipient::CreateNotificationRecipientRequest,
        notification_recipient::UpdateNotificationRecipientRequest,
    },
    services::notification_recipient,
};

pub async fn list(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
) -> Result<Json<Vec<NotificationRecipient>>, AppError> {

    let recipients = notification_recipient::list(&state.pool).await?;

    Ok(Json(recipients))
}

pub async fn create(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Json(request): Json<CreateNotificationRecipientRequest>,
) -> Result<Json<NotificationRecipient>, AppError> {

    let recipient =
        notification_recipient::create(
            &state.pool,
            request,
        )
        .await?;

    Ok(Json(recipient))
}

pub async fn update(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(id): Path<i32>,
    Json(request): Json<UpdateNotificationRecipientRequest>,
) -> Result<Json<NotificationRecipient>, AppError> {

    let recipient =
        notification_recipient::update(
            &state.pool,
            id,
            request,
        )
        .await?;

    Ok(Json(recipient))
}