mod config;
mod db;

use axum::{extract::State, http::StatusCode, response::Json, routing::get, Router};
use diesel::RunQueryDsl;
use std::net::SocketAddr;
use std::{env, net::Ipv4Addr};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;
use db::{create_user, models::User};

pub struct AppState {
    database: db::Pool,
    config: Config,
}

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

    let state = AppState {
        database: pool.clone(),
        config: config.clone(),
    };

    let address: SocketAddr = format!("{}:{}", config.server.address, config.server.port)
        .parse()
        .unwrap(); //SocketAddr::from((Ipv4Addr::UNSPECIFIED, 54600));

    let lister = tokio::net::TcpListener::bind(&address).await.unwrap();

    let app = Router::new()
        .route("/api/v1/users", get(users))
        .with_state(pool);

    println!("listening on http://{}", address);

    axum::serve(lister, app.into_make_service())
        .await
        .map_err(internal_error)
        .unwrap();
}

async fn users(State(pool): State<db::Pool>) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    use db::schema::users::dsl::*;

    let conn = pool.get().await.unwrap();

    let result = conn
        .interact(move |conn| users.load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(Json(result))
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
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
