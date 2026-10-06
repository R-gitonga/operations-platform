use axum::{routing::get, Router};

use crate::{
    app_state::AppState,
    handlers::po_settings::{get_settings, update_settings},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/po-settings",
            get(get_settings).put(update_settings),
        )
}