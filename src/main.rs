use axum::{http::StatusCode, response::IntoResponse, routing::get, routing::Router};

async fn hello_world() -> impl IntoResponse {
    (StatusCode::OK, "hello, world!")
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/api/hello", get(hello_world));

    Ok(router.into())
}
