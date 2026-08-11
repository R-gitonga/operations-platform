use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    authenticated_user::AuthenticatedUser,
    app_state::AppState,
    errors::app_error::AppError,
    models::user::User,
    services::users,
};

#[derive(Debug, serde::Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateUserRequest {
    pub name: String,
    pub email: String,
    pub role: String,
    pub active: bool,
}

pub async fn list_users(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<Json<Vec<User>>, AppError> {
    if user.0.role != "admin" {
        return Err(AppError::Forbidden);
    }
    let users = users::list(&state.pool).await?;

    Ok(Json(users))
}

pub async fn get_user(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
) -> Result<Json<User>, AppError> {
    if user.0.role != "admin" {
        return Err(AppError::Forbidden);
    }
    let user = users::get(&state.pool, id).await?;

    Ok(Json(user))
}

pub async fn create_user(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<User>, AppError> {
    if user.0.role != "admin" {
        return Err(AppError::Forbidden);
    }

    let user = users::create(
        &state.pool,
        &request.name,
        &request.email,
        &request.password,
        &request.role,
    )
    .await?;

    Ok(Json(user))
}

pub async fn update_user(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<Json<User>, AppError> {
    if user.0.role != "admin" {
        return Err(AppError::Forbidden);
    }
    let user = users::update(
        &state.pool,
        id,
        &request.name,
        &request.email,
        &request.role,
        request.active,
    )
    .await?;

    Ok(Json(user))
}

pub async fn deactivate_user(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
) -> Result<Json<User>, AppError> {
    if user.0.role != "admin" {
        return Err(AppError::Forbidden);
    }
    let user = users::deactivate(
        &state.pool,
        id,
    )
    .await?;

    Ok(Json(user))
}