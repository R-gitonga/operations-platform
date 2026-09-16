use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    authenticated_user::AuthenticatedUser,
    errors::app_error::AppError,
    models::{
        create_supplier_request::CreateSupplierRequest,
        supplier::Supplier,
        update_supplier_request::UpdateSupplierRequest,
    },
    services::supplier,
};

pub async fn get_active_suppliers(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
) -> Result<Json<Vec<Supplier>>, AppError> {
    let suppliers =
        supplier::find_active(&state.pool)
            .await?;

    Ok(Json(suppliers))
}

pub async fn get_suppliers(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<Json<Vec<Supplier>>, AppError> {
    if user.0.role != "admin" {
        return Err(AppError::Forbidden);
    }

    let suppliers =
        supplier::find_all(&state.pool)
            .await?;

    Ok(Json(suppliers))
}

pub async fn get_supplier(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
) -> Result<Json<Supplier>, AppError> {
    if user.0.role != "admin" {
        return Err(AppError::Forbidden);
    }

    let supplier =
        supplier::find_by_id(
            &state.pool,
            id,
        )
        .await?;

    Ok(Json(supplier))
}

pub async fn create_supplier(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<CreateSupplierRequest>,
) -> Result<Json<Supplier>, AppError> {
    if user.0.role != "admin" {
        return Err(AppError::Forbidden);
    }

    let supplier =
        supplier::create(
            &state.pool,
            &payload,
        )
        .await?;

    Ok(Json(supplier))
}

pub async fn update_supplier(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateSupplierRequest>,
) -> Result<Json<Supplier>, AppError> {
    if user.0.role != "admin" {
        return Err(AppError::Forbidden);
    }

    let supplier =
        supplier::update(
            &state.pool,
            id,
            &payload,
        )
        .await?;

    Ok(Json(supplier))
}

pub async fn activate_supplier(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
) -> Result<Json<Supplier>, AppError> {
    if user.0.role != "admin" {
        return Err(AppError::Forbidden);
    }

    let supplier =
        supplier::activate(
            &state.pool,
            id,
        )
        .await?;

    Ok(Json(supplier))
}

pub async fn deactivate_supplier(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
) -> Result<Json<Supplier>, AppError> {
    if user.0.role != "admin" {
        return Err(AppError::Forbidden);
    }

    let supplier =
        supplier::deactivate(
            &state.pool,
            id,
        )
        .await?;

    Ok(Json(supplier))
}