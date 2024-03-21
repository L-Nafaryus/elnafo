pub mod api;
pub mod config;
pub mod db;
pub mod error_handle;
pub mod state;

use axum::{
    extract::Path,
    http::{header::CONTENT_TYPE, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::config::Config;
use crate::error_handle::*;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    //init_tracing();
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let config = Config::new();
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.database.user,
        config.database.password,
        config.database.host,
        config.database.port,
        config.database.name
    );

    let pool = db::create_pool(database_url);

    db::run_migrations(&pool).await;

    let state = Arc::new(AppState {
        database: pool.clone(),
        config: config.clone(),
    });

    let address: SocketAddr = format!("{}:{}", config.server.address, config.server.port)
        .parse()
        .unwrap(); //SocketAddr::from((Ipv4Addr::UNSPECIFIED, 54600));

    let lister = tokio::net::TcpListener::bind(&address).await.unwrap();

    let app = Router::new()
        .route("/", get(home))
        .route("/user/login", get(user_login))
        .route("/assets/*file", get(static_handler))
        .nest("/api", api::v1::routes(state))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    println!("listening on http://{}", address);

    axum::serve(lister, app.into_make_service())
        .await
        .map_err(internal_error)
        .unwrap();
}

async fn home() -> impl IntoResponse {
    frontend::BaseTemplate { view: "app" }
}

async fn user_login() -> impl IntoResponse {
    frontend::BaseTemplate { view: "app" }
}

async fn user(Path(user): Path<String>) -> impl IntoResponse {}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    if path.starts_with("assets/") {
        path = path.replace("assets/", "");
    }

    StaticFile(path)
}

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match frontend::Assets::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                ([(CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
        }
    }
}
