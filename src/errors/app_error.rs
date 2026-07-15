use axum::{
    extract::multipart::MultipartError,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;


#[derive(Debug)]
pub enum AppError {
    NotFound,
    BadRequest(String),
    Sqlx(sqlx::Error),
    Multipart(MultipartError),
    Io(std::io::Error),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NotFound => write!(f, "Not found"),
            AppError::BadRequest(message) => {
                write!(f, "Bad Request: {}", message)
            }
            AppError::Sqlx(err) => {
                write!(f, "Database error: {}", err)
            } 
            AppError::Multipart(err) => {
                write!(f, "Multipart error: {}", err)
            }

            AppError::Io(err) => write!(f, "IO error: {}", err),
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

impl From<MultipartError> for AppError {
    fn from(err: MultipartError) -> Self {
        AppError::Multipart(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
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
            AppError::BadRequest(message) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": message})),
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

            AppError::Multipart(err) => {
                eprintln!("Multipart error: {:?}", err);

                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error": "Invalid file upload."
                    })),
                )
                    .into_response()
            }

            AppError::Io(err) => {
                eprintln!("IO error: {:?}", err);

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error": "File system error"
                    })),
                )
                    .into_response()
            }
        }
    }
}
