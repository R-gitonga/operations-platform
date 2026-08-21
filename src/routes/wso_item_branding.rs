use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::{
    app_state::AppState,
    handlers::wso_item_branding::{
        create_branding_requirement,
        delete_branding_requirement,
        get_branding_requirements,
    },
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/wso-items/{wso_item_id}/branding",
            get(get_branding_requirements)
                .post(create_branding_requirement),
        )
        .route(
            "/wso-item-branding/{id}",
            delete(delete_branding_requirement),
        )
}