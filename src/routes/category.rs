use axum::{
    routing::get,
    Router,
};

use crate::{
    app_state::AppState,
    handlers::category::{
        create_category,
        delete_category,
        get_categories,
        get_category,
        get_category_items,
        update_category,
    },
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/categories",
            get(get_categories).post(create_category),
        )
        .route(
            "/categories/{id}",
            get(get_category)
                .put(update_category)
                .delete(delete_category),
        )
        .route(
            "/categories/{id}/items",
            get(get_category_items),
        )
}