use argon2::Argon2;
use argon2::{PasswordHash, PasswordVerifier};
use axum::extract::Path;
use axum::Extension;
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use serde_json::json;
use std::sync::Arc;

use crate::db::user::User;
use crate::state::AppState;

use super::errors::AuthError;
use super::token::TokenClaims;

#[derive(serde::Deserialize)]
pub struct RegisterUser {
    pub login: String,
    pub password: String,
    pub email: String,
}

#[derive(serde::Serialize)]
pub struct FilteredUser {
    pub id: String,
    pub login: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(serde::Deserialize)]
pub struct RemoveUser {
    pub id: String,
}

impl FilteredUser {
    pub fn from(user: &User) -> Self {
        FilteredUser {
            id: user.id.to_string(),
            login: user.login.to_string(),
            name: user.name.to_owned(),
            email: user.email.to_owned(),
            is_admin: user.is_admin,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterUser>,
) -> Result<impl IntoResponse, AuthError<impl std::error::Error>> {
    let user = User::register(
        &state.database,
        body.login.to_owned(),
        body.password,
        body.login, //body.name,
        body.email,
        false, //body.is_admin,
    )
    .await
    .map_err(AuthError::InternalError)?;

    Ok(Json(json!({
        "status": StatusCode::OK.to_string(),
        "user": FilteredUser::from(&user)
    })))
}

pub async fn remove(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RemoveUser>,
) -> Result<impl IntoResponse, AuthError<impl std::error::Error>> {
    let user = User::find(
        &state.database,
        User::by_id(uuid::Uuid::parse_str(&body.id).map_err(|_| AuthError::InvalidCredentials)?),
    )
    .await
    .map_err(AuthError::InternalError)?;

    let user = match user {
        Some(user) => user,
        None => return Err(AuthError::MissingUser),
    };

    User::remove(&state.database, user)
        .await
        .map_err(|_| AuthError::InternalE)?;

    Ok(Json(json!({"status": StatusCode::OK.to_string()})))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LoginUser>,
) -> Result<impl IntoResponse, AuthError<impl std::error::Error>> {
    let user = User::find(&state.database, User::by_email(body.email))
        .await
        .map_err(AuthError::InternalError)?;

    let user = match user {
        Some(user) => user,
        None => return Err(AuthError::InvalidCredentials),
    };

    if !match PasswordHash::new(&user.hashed_password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    } {
        return Err(AuthError::InvalidCredentials);
    }

    let token = TokenClaims::create(
        user.id.to_string(),
        state.config.jwt.secret.to_owned(),
        state.config.jwt.maxage,
    )
    .unwrap();

    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::None)
        .secure(true)
        .http_only(true);

    let mut response =
        Json(json!({"status": StatusCode::OK.to_string(), "token": token, "user": json!(FilteredUser::from(&user))})).into_response();
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)
}

pub async fn logout() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::None)
        .secure(true)
        .http_only(true);

    let mut response = Json(json!({"status": StatusCode::OK.to_string()})).into_response();
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)
}

pub async fn profile(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Option<uuid::Uuid>>,
    Path(login): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = User::find(&state.database, User::by_login(login))
        .await
        .map_err(|_| ())
        .unwrap();

    let response = if let Some(user) = user {
        json!({"status": StatusCode::OK.to_string(), "user": json!(FilteredUser::from(&user))})
    } else {
        json!({"status": StatusCode::NOT_FOUND.to_string()})
    };

    Ok(Json(response))
}

pub async fn current(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Option<uuid::Uuid>>,
) -> Result<impl IntoResponse, AuthError<impl std::error::Error>> {
    let user = get_user(state, user_id).await?;

    Ok(Json(
        json!({"status": StatusCode::OK.to_string(), "user": json!(FilteredUser::from(&user))}),
    ))
}

async fn get_user(
    state: Arc<AppState>,
    user_id: Option<uuid::Uuid>,
) -> Result<User, AuthError<impl std::error::Error>> {
    let user = if let Some(user_id) = user_id {
        User::find(&state.database, User::by_id(user_id))
            .await
            .map_err(AuthError::InternalError)
    } else {
        Err(AuthError::InvalidCredentials)
    };

    let user = user?.ok_or_else(|| AuthError::MissingUser)?;

    Ok(user)
}
