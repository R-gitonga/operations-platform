use axum::{
    routing::{get, patch},
    Router,
};

use crate::{
    app_state::AppState,
    handlers::supplier::{
        activate_supplier,
        create_supplier,
        deactivate_supplier,
        get_active_suppliers,
        get_supplier,
        get_suppliers,
        update_supplier,
    },
};

pub fn routes() -> Router<AppState> {
    Router::new()
        // Authenticated users can see active suppliers (e.g. for
        // the supplier picker when creating a PO).
        .route(
            "/suppliers",
            get(get_active_suppliers)
                .post(create_supplier),
        )

        // Admin configuration endpoint.
        .route(
            "/suppliers/all",
            get(get_suppliers),
        )

        .route(
            "/suppliers/{id}",
            get(get_supplier)
                .put(update_supplier),
        )

        .route(
            "/suppliers/{id}/activate",
            patch(activate_supplier),
        )

        .route(
            "/suppliers/{id}/deactivate",
            patch(deactivate_supplier),
        )
}