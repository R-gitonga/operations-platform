use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::{
    app_state::AppState,
    handlers::{
        branding_location::{
            activate_branding_location,
            create_branding_location,
            deactivate_branding_location,
            get_active_branding_locations,
            get_branding_location,
            get_branding_locations,
            update_branding_location,
        },
        branding_type::{
            activate_branding_type,
            create_branding_type,
            deactivate_branding_type,
            get_active_branding_types,
            get_branding_type,
            get_branding_types,
            update_branding_type,
        },
    },
};

pub fn routes() -> Router<AppState> {
    Router::new()
        // -------------------------------------------------
        // Branding types
        // -------------------------------------------------

        // Authenticated users can see active types.
        .route(
            "/branding/types",
            get(get_active_branding_types)
                .post(create_branding_type),
        )

        // Admin configuration endpoint.
        .route(
            "/branding/types/all",
            get(get_branding_types),
        )

        .route(
            "/branding/types/{id}",
            get(get_branding_type)
                .put(update_branding_type),
        )

        .route(
            "/branding/types/{id}/activate",
            patch(activate_branding_type),
        )

        .route(
            "/branding/types/{id}/deactivate",
            patch(deactivate_branding_type),
        )

        // -------------------------------------------------
        // Branding locations
        // -------------------------------------------------

        // Authenticated users can see active locations.
        .route(
            "/branding/locations",
            get(get_active_branding_locations)
                .post(create_branding_location),
        )

        // Admin configuration endpoint.
        .route(
            "/branding/locations/all",
            get(get_branding_locations),
        )

        .route(
            "/branding/locations/{id}",
            get(get_branding_location)
                .put(update_branding_location),
        )

        .route(
            "/branding/locations/{id}/activate",
            patch(activate_branding_location),
        )

        .route(
            "/branding/locations/{id}/deactivate",
            patch(deactivate_branding_location),
        )
}