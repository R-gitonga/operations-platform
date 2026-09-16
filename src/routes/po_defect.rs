use axum::{
    routing::{get, patch},
    Router,
};

use crate::{
    app_state::AppState,
    handlers::po_defect::{get_defects, report_defect, resolve_defect},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/po-line-items/{id}/defects",
            get(get_defects).post(report_defect),
        )
        .route(
            "/po-defects/{id}/resolve",
            patch(resolve_defect),
        )
}