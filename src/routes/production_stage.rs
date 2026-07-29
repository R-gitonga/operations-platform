use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    app_state::AppState,
    handlers::production_stage::{
        change_stage,
        get_stage_history,
        list_production_stages,
    },
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/production-stages",
            get(list_production_stages),
        )
        .route(
            "/wso-items/{id}/stage-history",
            get(get_stage_history),
        )
        .route(
            "/wso-items/{id}/stage",
            post(change_stage),
        )
}