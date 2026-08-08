use axum::{
    routing::post,
    Router,
};

use crate::{
    app_state::AppState,
    handlers::auth::login,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/auth/login",
            post(login),
        )
}