use axum::{
    routing::{get, patch},
    Router,
};

use crate::{
    app_state::AppState,
    handlers::settings::{
        get_notification_events,
        get_notification_settings,
        update_notification_setting,

    },
};
pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/settings/notifications/events",
            get(get_notification_events),
        )
        .route(
            "/settings/notifications",
            get(get_notification_settings),
        )
        .route(
            "/settings/notifications/{id}",
            patch(update_notification_setting),
        )
}
