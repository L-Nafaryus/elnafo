use std::sync::Arc;

use axum::{
    http::{
        header::{self, ACCEPT_ENCODING, CONTENT_TYPE, ORIGIN},
        Method, StatusCode, Uri,
    },
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tower_http::{
    compression::{CompressionLayer, DefaultPredicate},
    cors::CorsLayer,
};

use crate::{config::Config, state::AppState};

pub fn routes(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_headers(vec![ORIGIN, CONTENT_TYPE, ACCEPT_ENCODING])
        .allow_origin([
            "http://localhost:54600".parse().unwrap(),
            "http://localhost:5173".parse().unwrap(),
        ])
        .allow_credentials(true);

    let compression = CompressionLayer::new().gzip(true);

    Router::new()
        .route("/assets/*file", get(assets))
        .route("/avatars/*avatar_id", get(avatars).layer(compression))
        .layer(cors)
}

async fn assets(uri: Uri) -> Result<impl IntoResponse, ResourceError> {
    let path = uri.path().trim_start_matches("/assets/").to_string();

    match elnafo_frontend::Assets::get(&path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            Ok(([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response())
        }
        None => Err(ResourceError::NotFound),
    }
}

async fn avatars(uri: Uri) -> Result<impl IntoResponse, ResourceError> {
    let avatar_id = uri.path().trim_start_matches("/avatars/").to_string();
    let path = Config::data_dir().unwrap().join("avatars").join(avatar_id);

    let reader = image::io::Reader::open(path.clone())
        .map_err(|_| ResourceError::NotFound)?
        .with_guessed_format()
        .map_err(|_| ResourceError::BadFormat)?;
    let format = reader.format();

    let mime = format.map_or("application/octet-stream", |f| f.to_mime_type());
    let content = reader.decode().map_err(|_| ResourceError::BadContent)?;

    let mut bytes: Vec<u8> = Vec::new();
    let _ = match format {
        Some(format) => content
            .write_to(&mut std::io::Cursor::new(&mut bytes), format)
            .map_err(|_| ResourceError::BadContent),
        None => return Err(ResourceError::BadFormat),
    };

    Ok(([(header::CONTENT_TYPE, mime)], bytes).into_response())
}

#[derive(Debug)]
pub enum ResourceError {
    NotFound,
    NotExists,
    BadFormat,
    BadContent,
}

impl std::error::Error for ResourceError {}

impl std::fmt::Display for ResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "Resource was not found"),
            Self::NotExists => write!(f, "Resource was not found"),
            Self::BadFormat => write!(f, "Cannot determine file format"),
            Self::BadContent => write!(f, "Failed to read a file content"),
        }
    }
}

impl IntoResponse for ResourceError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::NotExists => StatusCode::NO_CONTENT,
            Self::BadFormat | Self::BadContent => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, format!("{}", self)).into_response()
    }
}
