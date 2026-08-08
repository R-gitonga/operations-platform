use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::{
    app_state::AppState,
    handlers::users::{
        create_user,
        deactivate_user,
        get_user,
        list_users,
        update_user,
    },
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/users",
            get(list_users)
                .post(create_user),
        )
        .route(
            "/users/{id}",
            get(get_user)
                .put(update_user),
        )
        .route(
            "/users/{id}/deactivate",
            patch(deactivate_user),
        )
}