use axum::{http::StatusCode, Json};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "elnafo=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

pub fn internal_error<E>(err: E) -> (StatusCode, Json<serde_json::Value>)
where
    E: std::error::Error,
{
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({
        "status": "fail",
        "message": err.to_string()
        })),
    )
}
