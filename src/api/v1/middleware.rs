use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::CookieJar;

use crate::{db::user::User, state::AppState};

use super::errors::AuthError;
use super::token::TokenClaims;

pub async fn jwt(
    cookie_jar: CookieJar,
    State(state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, AuthError<impl std::error::Error>> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| auth_value.strip_prefix("Bearer "))
                .map(|auth_token| auth_token.to_owned())
        });

    let token = token.ok_or_else(|| AuthError::MissingToken)?;
    let claims = TokenClaims::validate(token, state.config.jwt.secret.to_owned())
        .map_err(|_| AuthError::InvalidToken)?;

    let user_id = uuid::Uuid::parse_str(&claims.sub).map_err(|_| AuthError::InvalidToken)?;

    let user = User::find(&state.database, User::by_id(user_id))
        .await
        .map_err(AuthError::InternalError)?;

    let user = user.ok_or_else(|| AuthError::MissingUser)?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

pub async fn jwt_auth(
    cookie_jar: CookieJar,
    State(state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| auth_value.strip_prefix("Bearer "))
                .map(|auth_token| auth_token.to_owned())
        });

    let user_id = token
        .and_then(|token| TokenClaims::validate(token, state.config.jwt.secret.to_owned()).ok())
        .and_then(|claims| uuid::Uuid::parse_str(&claims.sub).ok());

    req.extensions_mut().insert(user_id);
    Ok(next.run(req).await)
}
