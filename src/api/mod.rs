pub mod doc;
pub mod errors;
pub mod middleware;
pub mod token;
pub mod user;

use std::sync::Arc;

use axum::{
    extract::DefaultBodyLimit,
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
        .route("/healthcheck", get(healthcheck))
        .route("/user/all", get(user::all))
        .route("/user/register", post(user::register))
        .route("/user/remove", post(user::remove))
        .route("/user/login", post(user::login))
        .route("/user/logout", get(user::logout))
        .route(
            "/user/current",
            get(user::current).route_layer(jwt.to_owned()),
        )
        .route(
            "/user/:login",
            get(user::profile).route_layer(jwt.to_owned()),
        )
        .route(
            "/user/avatar",
            post(user::avatar)
                .route_layer(jwt)
                .layer(DefaultBodyLimit::max(10 * 10000)),
        )
        .layer(cors)
        .fallback(fallback)
        .with_state(state)
}

#[utoipa::path(get, path = "/api/healthcheck", responses((status = 200, body = String)))]
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
