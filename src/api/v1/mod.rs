pub mod errors;
pub mod middleware;
pub mod token;
pub mod user;

use std::sync::Arc;

use axum::{
    http::{header::*, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use tower_http::cors::CorsLayer;

use crate::state::AppState;

pub fn routes(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(vec![ORIGIN, AUTHORIZATION, ACCEPT, CONTENT_TYPE, COOKIE])
        .allow_origin([
            "http://localhost:54600".parse().unwrap(),
            "http://localhost:5173".parse().unwrap(),
        ])
        .allow_credentials(true);

    let jwt = axum::middleware::from_fn_with_state(state.to_owned(), middleware::jwt_auth);

    Router::new()
        .route("/v1/healthcheck", get(healthcheck))
        .route("/v1/user/register", post(user::register))
        .route("/v1/user/remove", post(user::remove))
        .route("/v1/user/login", post(user::login))
        .route("/v1/user/logout", get(user::logout))
        .route("/v1/user/profile", get(user::profile).route_layer(jwt))
        .layer(cors)
        .fallback(fallback)
        .with_state(state)
}

pub async fn healthcheck() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "status": StatusCode::OK.to_string(),
        })),
    )
}

pub async fn fallback() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "status": StatusCode::NOT_FOUND.to_string(),
        })),
    )
}
