use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    authenticated_user::AuthenticatedUser,
    errors::app_error::AppError,
    models::{
        branding_type::BrandingType,
        create_branding_type_request::CreateBrandingTypeRequest,
        update_branding_type_request::UpdateBrandingTypeRequest,
    },
    services::branding_type,
};

pub async fn get_active_branding_types(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
) -> Result<Json<Vec<BrandingType>>, AppError> {
    let types =
        branding_type::find_active(&state.pool)
            .await?;

    Ok(Json(types))
}

pub async fn get_branding_types(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<Json<Vec<BrandingType>>, AppError> {
    if user.0.role != "admin" {
        return Err(AppError::Forbidden);
    }

    let types =
        branding_type::find_all(&state.pool)
            .await?;

    Ok(Json(types))
}

pub async fn get_branding_type(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
) -> Result<Json<BrandingType>, AppError> {
    if user.0.role != "admin" {
        return Err(AppError::Forbidden);
    }

    let branding_type =
        branding_type::find_by_id(
            &state.pool,
            id,
        )
        .await?;

    Ok(Json(branding_type))
}

pub async fn create_branding_type(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<CreateBrandingTypeRequest>,
) -> Result<Json<BrandingType>, AppError> {
    if user.0.role != "admin" {
        return Err(AppError::Forbidden);
    }

    let branding_type =
        branding_type::create(
            &state.pool,
            &payload,
        )
        .await?;

    Ok(Json(branding_type))
}

pub async fn update_branding_type(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateBrandingTypeRequest>,
) -> Result<Json<BrandingType>, AppError> {
    if user.0.role != "admin" {
        return Err(AppError::Forbidden);
    }

    let branding_type =
        branding_type::update(
            &state.pool,
            id,
            &payload,
        )
        .await?;

    Ok(Json(branding_type))
}

pub async fn activate_branding_type(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
) -> Result<Json<BrandingType>, AppError> {
    if user.0.role != "admin" {
        return Err(AppError::Forbidden);
    }

    let branding_type =
        branding_type::activate(
            &state.pool,
            id,
        )
        .await?;

    Ok(Json(branding_type))
}

pub async fn deactivate_branding_type(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
) -> Result<Json<BrandingType>, AppError> {
    if user.0.role != "admin" {
        return Err(AppError::Forbidden);
    }

    let branding_type =
        branding_type::deactivate(
            &state.pool,
            id,
        )
        .await?;

    Ok(Json(branding_type))
}