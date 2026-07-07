use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    app_state::AppState,
    handlers::line_item::{
        create_line_item,
        delete_line_item,
        get_line_item,
        get_line_items,
        update_line_item,
    },
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/wso/{id}/line-items",
            post(create_line_item).get(get_line_items),
        )
        .route(
            "/line-items/{id}",
            get(get_line_item).put(update_line_item).delete(delete_line_item),
        )
}
