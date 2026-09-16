use axum::{
    routing::{get, post, put},
    Router,
};

use crate::{
    app_state::AppState,
    handlers::{
        po_line_item::{add_line_item, update_line_item},
        po_item_note::{add_note, get_notes},
    },
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/po-items/{id}/line-items",
            post(add_line_item),
        )

        .route(
            "/po-line-items/{id}",
            put(update_line_item),
        )

        .route(
            "/po-items/{id}/notes",
            post(add_note)
                .get(get_notes),
        )
}