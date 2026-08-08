use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};
use axum_extra::extract::cookie::CookieJar;

use crate::{
    app_state::AppState,
    errors::app_error::AppError,
    models::user::User,
    repositories::user,
    services::auth_token,
};

pub struct AuthenticatedUser(pub User);

impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let cookies = CookieJar::from_headers(&parts.headers);

        let token = cookies
            .get("auth_token")
            .map(|cookie| cookie.value())
            .ok_or(AppError::Unauthorized)?;

        let claims = auth_token::validate_token(
            token,
            &state.config.jwt_secret,
        )
        .map_err(|_| AppError::Unauthorized)?;

        let user = user::find_by_id(&state.pool, claims.sub)
            .await
            .map_err(|_| AppError::Unauthorized)?;

        if !user.active {
            return Err(AppError::Unauthorized);
        }

        Ok(Self(user))
    }
}
