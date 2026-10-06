use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    app_state::AppState,
    handlers::auth::{bootstrap_admin, forgot_password, login, logout, me, reset_password},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/bootstrap-admin", post(bootstrap_admin))
        .route("/auth/forgot-password", post(forgot_password))
        .route("/auth/reset-password", post(reset_password))
        .route("/auth/logout", post(logout))
        .route("/auth/me", get(me))
}