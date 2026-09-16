use axum::{
    routing::post,
    Router,
};

use crate::{
    app_state::AppState,
    handlers::po_receipt::{create_receipt, get_receipts},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/purchase-orders/{id}/receipts",
            post(create_receipt)
                .get(get_receipts),
        )
}