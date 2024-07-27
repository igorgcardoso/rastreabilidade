use axum::{
    http::{header::InvalidHeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    message: String,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    ValidationError(#[from] validator::ValidationErrors),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    BadRequest(String),
    #[error("An internal error occurred.")]
    InternalServer,
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self::NotFound("Resource not found".to_string()),
            sqlx::Error::Database(db_err) => {
                tracing::error!("Database error: {:?}", db_err);
                Self::BadRequest(format!("{}", db_err))
            }
            _ => {
                tracing::error!("Database error: {:?}", err);
                Self::InternalServer
            }
        }
    }
}

impl From<InvalidHeaderValue> for AppError {
    fn from(value: InvalidHeaderValue) -> Self {
        tracing::error!("Invalid header value: {:?}", value);
        Self::InternalServer
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::ValidationError(_) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ErrorResponse {
                    message: self.to_string(),
                }),
            ),
            AppError::NotFound(message) => (StatusCode::NOT_FOUND, Json(ErrorResponse { message })),
            AppError::BadRequest(message) => {
                (StatusCode::BAD_REQUEST, Json(ErrorResponse { message }))
            }
            AppError::InternalServer => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "An internal error occurred".to_string(),
                }),
            ),
        }
        .into_response()
    }
}
