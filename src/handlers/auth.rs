use axum::{
    extract::State,
    Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};

use crate::{
    app_state::AppState,
    errors::app_error::AppError,
    models::auth::{LoginRequest, LoginResponse},
    services::{
        auth,
        auth_token,
    },
};

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<
    (
        axum_extra::extract::cookie::CookieJar,
        Json<LoginResponse>,
    ),
    AppError,
> {
    let user =
        auth::authenticate(
            &state.pool,
            &request.email,
            &request.password,
        )
        .await?;

    let token =
        auth_token::create_token(
            user.id,
            &state.config.jwt_secret,
            state.config.jwt_expiration_hours,
        )?;

    let cookie =
        Cookie::build(("auth_token", token))
            .http_only(true)
            .same_site(SameSite::Lax)
            .path("/")
            .secure(false)
            .build();

    let jar =
        axum_extra::extract::cookie::CookieJar::new()
            .add(cookie);

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