use axum::{
    routing::post,
    Router,
};

use crate::{
    app_state::AppState,
    handlers::debug::send_test_notification,
};

pub fn routes() -> Router<AppState> {

    Router::new()
        .route(
            "/debug/notifications/send",
            post(send_test_notification),
        )
}