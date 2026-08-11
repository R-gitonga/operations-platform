use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::{
    app_state::AppState,
    handlers::line_item::{
        create_line_item, delete_line_item, get_line_item, get_line_items, receive_line_item,
        update_line_item,
    },
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/wso-items/{id}/line-items",
            post(create_line_item).get(get_line_items),
        )
        .route(
            "/line-items/{id}",
            get(get_line_item)
                .put(update_line_item)
                .delete(delete_line_item),
        )
        .route("/line-items/{id}/receive", patch(receive_line_item))
}
