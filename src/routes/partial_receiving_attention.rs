use axum::{
    routing::{put, get},
    Router,
};

use crate::{
    app_state::AppState,
    handlers::{
        partial_receiving_settings,
        partial_receiving_attention::{
        get_attention_required_partial_receipts,
    },

    }
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/partial-receiving/attention-required",
            get(get_attention_required_partial_receipts),
        )
        .route(
    "/partial-receiving/settings",
    get(partial_receiving_settings::get_settings)
        .put(partial_receiving_settings::update_settings),
)
}