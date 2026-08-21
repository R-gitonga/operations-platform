use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    authenticated_user::AuthenticatedUser,
    errors::app_error::AppError,
    models::{
        branding_location::BrandingLocation,
        create_branding_location_request::CreateBrandingLocationRequest,
        update_branding_location_request::UpdateBrandingLocationRequest,
    },
    services::branding_location,
};

pub async fn get_active_branding_locations(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
) -> Result<Json<Vec<BrandingLocation>>, AppError> {
    let locations =
        branding_location::find_active(&state.pool)
            .await?;

    Ok(Json(locations))
}

pub async fn get_branding_locations(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<Json<Vec<BrandingLocation>>, AppError> {
    if user.0.role != "admin" {
        return Err(AppError::Forbidden);
    }

    let locations =
        branding_location::find_all(&state.pool)
            .await?;

    Ok(Json(locations))
}

pub async fn get_branding_location(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
) -> Result<Json<BrandingLocation>, AppError> {
    if user.0.role != "admin" {
        return Err(AppError::Forbidden);
    }

    let location =
        branding_location::find_by_id(
            &state.pool,
            id,
        )
        .await?;

    Ok(Json(location))
}

pub async fn create_branding_location(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<CreateBrandingLocationRequest>,
) -> Result<Json<BrandingLocation>, AppError> {
    if user.0.role != "admin" {
        return Err(AppError::Forbidden);
    }

    let location =
        branding_location::create(
            &state.pool,
            &payload,
        )
        .await?;

    Ok(Json(location))
}

pub async fn update_branding_location(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateBrandingLocationRequest>,
) -> Result<Json<BrandingLocation>, AppError> {
    if user.0.role != "admin" {
        return Err(AppError::Forbidden);
    }

    let location =
        branding_location::update(
            &state.pool,
            id,
            &payload,
        )
        .await?;

    Ok(Json(location))
}

pub async fn activate_branding_location(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
) -> Result<Json<BrandingLocation>, AppError> {
    if user.0.role != "admin" {
        return Err(AppError::Forbidden);
    }

    let location =
        branding_location::activate(
            &state.pool,
            id,
        )
        .await?;

    Ok(Json(location))
}

pub async fn deactivate_branding_location(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
) -> Result<Json<BrandingLocation>, AppError> {
    if user.0.role != "admin" {
        return Err(AppError::Forbidden);
    }

    let location =
        branding_location::deactivate(
            &state.pool,
            id,
        )
        .await?;

    Ok(Json(location))
}