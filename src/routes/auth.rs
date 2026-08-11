use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    app_state::AppState,
    handlers::auth::{login, logout, me},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
        .route("/auth/me", get(me))
}
