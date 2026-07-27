use std::collections::HashMap;

use axum::{
    extract::State,
    Json,
};

use crate::{
    app_state::AppState,
    errors::app_error::AppError,
    models::{
        notification_context::NotificationContext,
        test_notification_request::TestNotificationRequest,
    },
    services::notifications,
};

pub async fn send_test_notification(
    State(state): State<AppState>,
    Json(request): Json<TestNotificationRequest>,
) -> Result<(), AppError> {

    let mut variables = HashMap::new();

    variables.insert(
        "wso_number".to_string(),
        request.wso_number,
    );

    variables.insert(
        "department".to_string(),
        request.department,
    );

    variables.insert(
        "description".to_string(),
        request.description,
    );

    let context = NotificationContext {

        event_code: request.event_code,

        actor_name: request.actor_name,

        actor_email: request.actor_email,

        variables,
    };

    notifications::dispatch(
        &state.pool,
        context,
    )
    .await?;

    Ok(())
}