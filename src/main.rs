pub mod api;
pub mod config;
pub mod db;
pub mod resources;
pub mod state;

use axum::{http::Uri, response::IntoResponse, routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

use crate::config::Config;
use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let config = match Config::open(Config::data_dir()?.join("config.toml").as_path()) {
        Ok(config) => config,
        Err(_) => Config::new(),
    };

    let pool = db::create_pool(config.database_url());

    db::run_migrations(&pool).await?;

    let state = Arc::new(AppState {
        database: pool.clone(),
        config: config.clone(),
    });

    let app = Router::new()
        .nest("/resources", resources::routes(state.clone()))
        .nest("/api", api::routes(state))
        .merge(
            RapiDoc::with_openapi("/api/openapi.json", api::doc::ApiDoc::openapi())
                .path("/api/rapidoc"),
        )
        .route("/", get(frontend_handler))
        .route("/*frontend", get(frontend_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let address: SocketAddr =
        format!("{}:{}", config.server.address, config.server.port).parse()?;

    let lister = tokio::net::TcpListener::bind(&address).await?;

    println!("listening on {}", address);

    axum::serve(lister, app.into_make_service()).await?;

    Ok(())
}

async fn frontend_handler(_: Uri) -> impl IntoResponse {
    elnafo_frontend::BaseTemplate { view: "app" }
}
