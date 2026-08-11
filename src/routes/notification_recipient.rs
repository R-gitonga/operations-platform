use axum::{
    routing::{get, patch},
    Router,
};

use crate::{
    app_state::AppState,
    handlers::notification_recipient,
};

pub fn routes() -> Router<AppState> {
    Router::new()
    .route(
    "/settings/notifications/recipients",
    get(notification_recipient::list)
        .post(notification_recipient::create),
)
.route(
    "/settings/notifications/recipients/{id}",
    patch(notification_recipient::update),
)
}
