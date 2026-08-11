use axum::{
    routing::{get, patch, post, put},
    Router,
};

use crate::{
    app_state::AppState,
    handlers::production_stage::{
        change_stage, create_production_stage, deactivate_production_stage, get_production_stage,
        get_stage_history, get_stage_items, list_production_stages, update_production_stage,
    },
};

pub fn routes() -> Router<AppState> {
    Router::new()
        //-------------------------------------------------
        // Production Stage Settings
        //-------------------------------------------------
        .route(
            "/production-stages",
            get(list_production_stages).post(create_production_stage),
        )
        .route(
            "/production-stages/{id}",
            get(get_production_stage).put(update_production_stage),
        )
        .route(
            "/production-stages/{id}/deactivate",
            patch(deactivate_production_stage),
        )
        //-------------------------------------------------
        // Production Queue
        //-------------------------------------------------
        .route("/production-stages/{id}/items", get(get_stage_items))
        //-------------------------------------------------
        // Stage History
        //-------------------------------------------------
        .route("/wso-items/{id}/stage-history", get(get_stage_history))
        .route("/wso-items/{id}/stage", post(change_stage))
}
