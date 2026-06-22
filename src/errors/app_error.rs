use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    Sqlx(sqlx::Error),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NotFound => write!(f, "Not found"),
            AppError::Sqlx(err) => write!(f, "Database error: {}", err),
        }
    }
}

impl std::error::Error for AppError {}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound,
            err => AppError::Sqlx(err),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Not found"})),
            )
                .into_response(),
            AppError::Sqlx(err) => {
                eprintln!("database error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Internal server error"})),
                )
                    .into_response()
            }
        }
    }
}
