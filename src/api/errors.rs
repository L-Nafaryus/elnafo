use std::any::Any;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::db::errors::DatabaseError;

use super::user::UserError;

#[derive(Debug, utoipa::ToSchema)]
pub enum ApiError {
    Database(DatabaseError),
    AuthError(AuthError),
    ReadContent,
    Query(UserError),
}

impl std::error::Error for ApiError {}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Database(ref e) => e.fmt(f),
            Self::AuthError(e) => write!(f, "Authentication error occured: {}", e),
            Self::ReadContent => write!(f, "Failed to read body content"),
            Self::Query(ref e) => e.fmt(f),
        }
    }
}

impl From<DatabaseError> for ApiError {
    fn from(e: DatabaseError) -> Self {
        Self::Database(e)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::AuthError(_) => StatusCode::UNAUTHORIZED,
            Self::ReadContent => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Query(ref e) => match e {
                UserError::Exists => StatusCode::CONFLICT,
                UserError::HashPassword | UserError::ParseUuid => StatusCode::INTERNAL_SERVER_ERROR,
                UserError::MissedCredentials
                | UserError::InvalidCredentials
                | UserError::Unauthorized => StatusCode::UNAUTHORIZED,
                UserError::NotFound => StatusCode::NOT_FOUND,
            },
        };

        (status, format!("{}", self)).into_response()
    }
}

#[derive(Debug)]
pub enum AuthError {
    MissingCredentials,
    InvalidCredentials,
    MissingToken,
    InvalidToken,
    MissingUser,
}

impl std::error::Error for AuthError {}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingCredentials => write!(f, "Missing credentials"),
            Self::InvalidCredentials => write!(f, "Invalid credentials"),
            Self::MissingToken => write!(f, "Missing token"),
            Self::InvalidToken => write!(f, "Invalid token"),
            Self::MissingUser => write!(f, "Missing user"),
        }
    }
}

impl From<AuthError> for ApiError {
    fn from(e: AuthError) -> Self {
        Self::AuthError(e)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Self::MissingCredentials | Self::MissingToken => StatusCode::BAD_REQUEST,
            Self::InvalidCredentials | Self::InvalidToken | Self::MissingUser => {
                StatusCode::UNAUTHORIZED
            }
        };

        (status, format!("{}", self)).into_response()
    }
}
