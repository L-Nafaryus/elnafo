pub mod api;
pub mod config;
pub mod db;
pub mod error_handle;
pub mod state;

use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    response::Json,
    routing::{get, post},
    Router,
};
use diesel::RunQueryDsl;
use std::net::SocketAddr;
use std::sync::Arc;
use std::{env, net::Ipv4Addr};

use crate::config::Config;
use crate::db::{create_user, models::User};
use crate::error_handle::*;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    init_tracing();

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
        .route("/api/v1/healthcheck", get(api::v1::healthcheck))
        .route("/api/v1/users", get(users))
        .route("/api/v1/register_user", post(api::v1::register_user))
        .route("/api/v1/login_user", post(api::v1::login_user))
        .route("/api/v1/logout_user", get(api::v1::logout_user))
        .route(
            "/api/v1/me",
            get(api::v1::me).route_layer(middleware::from_fn_with_state(
                state.clone(),
                api::v1::jwt_auth,
            )),
        )
        .with_state(state);

    println!("listening on http://{}", address);

    axum::serve(lister, app.into_make_service())
        .await
        .map_err(internal_error)
        .unwrap();
}

async fn users(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<User>>, (StatusCode, Json<serde_json::Value>)> {
    use db::schema::users::dsl::*;

    let conn = state.database.get().await.unwrap();

    let result = conn
        .interact(move |conn| users.load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(Json(result))
}

/*
    create_user(
        connection,
        "L-Nafaryus",
        "asdasd",
        "L-Nafaryus",
        "l.nafaryus@elnafo.ru",
        true,
    );

    let results = users
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

    println!("Found {} users", results.len());
*/