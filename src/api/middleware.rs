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

use crate::{
    db::{self, schema::users, user::User},
    state::AppState,
};

use super::errors::AuthError;
use super::{errors::ApiError, token::TokenClaims};

pub async fn jwt(
    cookie_jar: CookieJar,
    State(state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, ApiError> {
    use diesel::prelude::*;

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

    let token = token.ok_or(AuthError::MissingToken)?;
    let claims = TokenClaims::validate(token, state.config.jwt.secret.to_owned())
        .map_err(|_| AuthError::InvalidToken)?;

    let user_id = uuid::Uuid::parse_str(&claims.sub).map_err(|_| AuthError::InvalidToken)?;

    let user = db::execute(&state.database, move |conn| {
        users::table
            .into_boxed()
            .filter(users::id.eq(user_id))
            .first::<User>(conn)
            .optional()
    })
    .await?;

    let user = user.ok_or(AuthError::MissingUser)?;

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
