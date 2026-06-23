use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::{
    app_state::AppState,
    handlers::wso::{
        cancel_wso,
        create_wso,
        get_wso,
        get_wso_summary,
        get_wsos,
        update_wso,
    },
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/wso",
            post(create_wso)
                .get(get_wsos),
        )
        .route(
            "/wso/{id}",
            get(get_wso)
                .put(update_wso),
        )
        .route(
            "/wso/summary",
            get(get_wso_summary),
        )
        .route(
            "/wso/{id}/cancel",
            patch(cancel_wso),
        )
}