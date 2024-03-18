pub mod api;
pub mod config;
pub mod db;
pub mod error_handle;
pub mod state;

use axum::{
    extract::State,
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, ORIGIN},
        HeaderValue, Method, StatusCode, Uri,
    },
    middleware,
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use diesel::RunQueryDsl;
use std::net::SocketAddr;
use std::sync::Arc;
use std::{env, net::Ipv4Addr};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{self, TraceLayer},
};
use tracing::Level;

use crate::config::Config;
use crate::db::{create_user, models::User};
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

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(vec![ORIGIN, AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_origin("http://0.0.0.0:54600".parse::<HeaderValue>().unwrap()) //Any)
        .allow_credentials(true); //"http://localhost:5173".parse::<HeaderValue>().unwrap());

    let app = Router::new()
        .route("/", get(home))
        .route("/user/login", get(user_login))
        .route("/assets/*file", get(static_handler))
        .route("/api/v1/register_user", post(api::v1::register_user))
        .route("/api/v1/login_user", post(api::v1::login_user))
        .layer(cors)
        .route("/api/v1/healthcheck", get(api::v1::healthcheck))
        .route("/api/v1/users", get(users))
        .route("/api/v1/logout_user", get(api::v1::logout_user))
        .route(
            "/api/v1/me",
            get(api::v1::me).route_layer(middleware::from_fn_with_state(
                state.clone(),
                api::v1::jwt_auth,
            )),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(state);

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
    frontend::BaseTemplate { view: "signin" }
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
