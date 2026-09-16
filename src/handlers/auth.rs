use axum::{extract::State, Json};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use serde_json::json;

use crate::{
    app_state::AppState,
    authenticated_user::AuthenticatedUser,
    errors::app_error::AppError,
    models::auth::{
        ForgotPasswordRequest,
        LoginRequest,
        LoginResponse,
        ResetPasswordRequest,
    },
    services::{auth, auth_token, password_reset},
};

pub async fn me(AuthenticatedUser(user): AuthenticatedUser) -> Json<LoginResponse> {
    Json(LoginResponse {
        user_id: user.id,
        email: user.email,
        display_name: user.name,
        role: user.role,
    })
}

pub async fn logout() -> (CookieJar, Json<serde_json::Value>) {
    let clearing_cookie = Cookie::build(("auth_token", ""))
        .http_only(true)
        .same_site(SameSite::Lax)
        .path("/")
        .secure(false)
        .build();

    let jar = CookieJar::new().add(clearing_cookie);

    (jar, Json(json!({ "message": "Logged out" })))
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<(axum_extra::extract::cookie::CookieJar, Json<LoginResponse>), AppError> {
    let user = auth::authenticate(&state.pool, &request.email, &request.password).await?;

    let token = auth_token::create_token(
        user.id,
        &state.config.jwt_secret,
        state.config.jwt_expiration_hours,
    )?;

    let cookie = Cookie::build(("auth_token", token))
        .http_only(true)
        .same_site(SameSite::Lax)
        .path("/")
        .secure(false)
        .build();

    let jar = axum_extra::extract::cookie::CookieJar::new().add(cookie);

    Ok((
        jar,
        Json(LoginResponse {
            user_id: user.id,
            email: user.email,
            display_name: user.name,
            role: user.role,
        }),
    ))
}

pub async fn forgot_password(
    State(state): State<AppState>,
    Json(request): Json<ForgotPasswordRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    password_reset::request(&state.pool, &state.config, &request.email).await?;

    Ok(Json(json!({
        "message": "If an active account uses that email address, a password reset link has been sent."
    })))
}

pub async fn reset_password(
    State(state): State<AppState>,
    Json(request): Json<ResetPasswordRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    password_reset::reset(&state.pool, &request.token, &request.password).await?;

    Ok(Json(json!({
        "message": "Your password has been reset. You can now sign in."
    })))
}
