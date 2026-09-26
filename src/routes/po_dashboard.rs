use axum::{routing::get, Router};

use crate::{
    app_state::AppState,
    handlers::po_dashboard::{get_dashboard, get_overdue},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/purchase-orders/dashboard", get(get_dashboard))
        .route("/purchase-orders/dashboard/overdue", get(get_overdue))
}