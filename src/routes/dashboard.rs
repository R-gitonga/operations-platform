use axum::{
    routing::get,
    Router,
};

use crate::{
    app_state::AppState,
    handlers::dashboard::get_dashboard,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/dashboard",
            get(get_dashboard),
        )
}