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

    if let Some(req_number) = request.req_number {
    variables.insert(
        "req_number".to_string(),
        req_number,
    );
}


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