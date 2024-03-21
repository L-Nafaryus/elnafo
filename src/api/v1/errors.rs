use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AuthError<E> {
    InternalError(E),
    MissingCredentials,
    InvalidCredentials,
    MissingToken,
    InvalidToken,
    MissingUser,
}

impl<E: std::error::Error> IntoResponse for AuthError<E> {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Self::InternalError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::MissingCredentials => {
                (StatusCode::BAD_REQUEST, "Missing credentials".to_string())
            }
            Self::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string())
            }
            Self::MissingToken => (StatusCode::BAD_REQUEST, "Missing token".to_string()),
            Self::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token".to_string()),
            Self::MissingUser => (StatusCode::UNAUTHORIZED, "User not exists".to_string()),
        };

        Json(json!({
            "status": status.to_string(),
            "message": message
        }))
        .into_response()
    }
}

pub fn internal_error<E>(err: E) -> (StatusCode, Json<serde_json::Value>)
where
    E: std::error::Error,
{
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
            "message": err.to_string()
        })),
    )
}
