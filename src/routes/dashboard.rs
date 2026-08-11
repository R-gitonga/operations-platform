use axum::{
    routing::get,
    Router,
};

use crate::{
    app_state::AppState,
    handlers::dashboard::{get_attention_required, get_dashboard},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/dashboard",
            get(get_dashboard),
        )
        .route(
            "/dashboard/attention-required",
            get(get_attention_required),
        )
}
