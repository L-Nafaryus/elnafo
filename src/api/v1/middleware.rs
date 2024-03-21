use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;

use crate::{db::user::User, state::AppState};

use super::errors::AuthError;
use super::token::TokenClaims;

pub async fn jwt_auth(
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
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
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
