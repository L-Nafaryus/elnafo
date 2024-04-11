use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use argon2::{PasswordHash, PasswordVerifier};
use axum::body::Bytes;
use axum::extract::{Multipart, Path};
use axum::http::HeaderValue;
use axum::response::Response;
use axum::Extension;
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use rand_core::OsRng;
use std::collections::HashSet;
use std::sync::Arc;

use crate::config::Config;
use crate::state::AppState;
use crate::{
    db,
    db::schema::users,
    db::user::{NewUser, User},
};

use super::errors::ApiError;
use super::token::TokenClaims;

#[derive(Debug, utoipa::ToSchema)]
pub enum UserError {
    Exists,
    HashPassword,
    ParseUuid,
    MissedCredentials,
    InvalidCredentials,
    NotFound,
    Unauthorized,
}

impl std::error::Error for UserError {}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exists => write!(f, "User already exists"),
            Self::HashPassword => write!(f, "Failed to create a password hash"),
            Self::ParseUuid => write!(f, "Failed to parse user UUID"),
            Self::MissedCredentials => write!(f, "Missed user credentials"),
            Self::InvalidCredentials => write!(f, "Invalid user credentials"),
            Self::NotFound => write!(f, "User not found"),
            Self::Unauthorized => write!(f, "User is not authorized"),
        }
    }
}

pub mod schema {
    use crate::db::user;

    #[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
    pub struct NewUser {
        pub login: String,
        pub password: String,
        pub email: String,
    }

    #[derive(Debug, serde::Serialize, utoipa::ToSchema)]
    pub struct User {
        pub id: String,
        pub login: String,
        pub name: String,
        pub email: String,
        pub is_admin: bool,
        pub avatar: String,
    }

    #[derive(serde::Deserialize, utoipa::ToSchema)]
    pub struct RemoveUser {
        pub id: String,
    }

    #[derive(serde::Deserialize, utoipa::ToSchema)]
    pub struct LoginUser {
        pub email: Option<String>,
        pub login: Option<String>,
        pub password: String,
    }

    #[derive(serde::Deserialize, utoipa::ToSchema)]
    pub struct Avatar {
        pub content: String,
        pub mime: String,
    }

    impl User {
        pub fn from(user: &user::User) -> Self {
            User {
                id: user.id.to_string(),
                login: user.login.to_string(),
                name: user.name.to_owned(),
                email: user.email.to_owned(),
                is_admin: user.is_admin,
                avatar: user.avatar.to_owned(),
            }
        }
    }

    #[derive(utoipa::ToSchema)]
    pub struct Image {
        #[schema(value_type = String, format = Binary)]
        pub file_content: Vec<u8>,
    }
}

#[utoipa::path(get, path = "/api/user/all",
    responses((status = 200, body = [User]))
)]
pub async fn all(State(state): State<Arc<AppState>>) -> Result<Json<Vec<schema::User>>, ApiError> {
    use diesel::prelude::*;

    let users = db::execute(&state.database, move |conn| {
        users::table.select(User::as_select()).get_results(conn)
    })
    .await?
    .into_iter()
    .map(|ref user| schema::User::from(user))
    .collect::<Vec<schema::User>>();

    Ok(Json(users))
}

#[utoipa::path(post, path = "/api/user/register", 
    request_body = NewUser,
    responses((status = 200, body = User), (status = 500, body = ApiError))
)]
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(body): Json<schema::NewUser>,
) -> Result<Json<schema::User>, ApiError> {
    use diesel::prelude::*;

    let count = db::execute(&state.database, move |conn| {
        users::table.into_boxed().count().get_result::<i64>(conn)
    })
    .await?;

    let (login, email) = (body.login.clone(), body.email.clone());
    let user = db::execute(&state.database, move |conn| {
        users::table
            .into_boxed()
            .filter(users::login.eq(login).or(users::email.eq(email)))
            .first::<User>(conn)
            .optional()
    })
    .await?;

    if user.is_some() {
        return Err(ApiError::Query(UserError::Exists));
    }

    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &SaltString::generate(&mut OsRng))
        .map_err(|_| ApiError::Query(UserError::HashPassword))
        .map(|hash| hash.to_string())?;

    let new_user = NewUser {
        login: body.login.clone(),
        hashed_password,
        name: body.login,
        email: body.email,
        is_admin: count == 0,
        avatar: String::default(),
    };

    let user = db::execute(&state.database, move |conn| {
        diesel::insert_into(users::table)
            .values(new_user)
            .returning(User::as_returning())
            .get_result(conn)
    })
    .await?;

    Ok(Json(schema::User::from(&user)))
}

#[utoipa::path(post, path = "/api/user/remove",
    request_body = RemoveUser,
    responses((status = 200))
)]
pub async fn remove(
    State(state): State<Arc<AppState>>,
    Json(body): Json<schema::RemoveUser>,
) -> Result<(), ApiError> {
    use diesel::prelude::*;

    let uuid =
        uuid::Uuid::parse_str(&body.id).map_err(|_| ApiError::Query(UserError::ParseUuid))?;

    db::execute(&state.database, move |conn| {
        diesel::delete(users::table.filter(users::id.eq(uuid))).execute(conn)
    })
    .await?;

    Ok(())
}

#[utoipa::path(post, path = "/api/user/login",
    request_body = LoginUser,
    responses((status = 200, body = User), (status = "4XX", body = UserError), (status = 500, body = ApiError))
)]
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(body): Json<schema::LoginUser>,
) -> Result<impl IntoResponse, ApiError> {
    use diesel::prelude::*;

    let query = users::table.into_boxed().select(User::as_select());
    let query = if let Some(login) = body.login {
        query.filter(users::login.eq(login))
    } else if let Some(email) = body.email {
        query.filter(users::email.eq(email))
    } else {
        return Err(ApiError::Query(UserError::MissedCredentials));
    };

    let user = db::execute(&state.database, move |conn| {
        query.first::<User>(conn).optional()
    })
    .await?;

    let user = match user {
        Some(user) => user,
        None => return Err(ApiError::Query(UserError::InvalidCredentials)),
    };

    if !match PasswordHash::new(&user.hashed_password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    } {
        return Err(ApiError::Query(UserError::InvalidCredentials));
    }

    let token = TokenClaims::create(
        user.id.to_string(),
        state.config.jwt.secret.to_owned(),
        state.config.jwt.maxage,
    )
    .unwrap();

    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::None)
        .secure(true)
        .http_only(true);

    let mut response = Json(schema::User::from(&user)).into_response();
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)
}

#[utoipa::path(get, path = "/api/user/logout", responses((status = 200)))]
pub async fn logout() -> Result<axum::response::Response, ApiError> {
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::None)
        .secure(true)
        .http_only(true);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(
            header::SET_COOKIE,
            cookie.to_string().parse::<HeaderValue>().unwrap(),
        )
        .body(axum::body::Body::empty())
        .unwrap();

    Ok(response)
}

#[utoipa::path(get, path = "/api/user/{login}", 
    params(("login", Path,)), 
    responses((status = 404, body = UserError), (status = 500, body = ApiError))
)]
pub async fn profile(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Option<uuid::Uuid>>,
    Path(login): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    use diesel::prelude::*;

    // TODO: Current user priveleges
    let user = db::execute(&state.database, move |conn| {
        users::table
            .into_boxed()
            .filter(users::login.eq(login))
            .first::<User>(conn)
            .optional()
    })
    .await?;

    match user {
        Some(user) => Ok(Json(schema::User::from(&user))),
        None => Err(ApiError::Query(UserError::NotFound)),
    }
}

#[utoipa::path(get, path = "/api/user/current", 
    security(("token" = []))
)]
pub async fn current(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Option<uuid::Uuid>>,
) -> Result<impl IntoResponse, ApiError> {
    use diesel::prelude::*;

    let uuid = match user_id {
        Some(user_id) => user_id,
        None => return Err(ApiError::Query(UserError::Unauthorized)),
    };

    let user = db::execute(&state.database, move |conn| {
        users::table
            .into_boxed()
            .filter(users::id.eq(uuid))
            .first::<User>(conn)
            .optional()
    })
    .await?;

    match user {
        Some(user) => Ok(Json(schema::User::from(&user))),
        None => Err(ApiError::Query(UserError::NotFound)),
    }
}

#[axum::debug_handler]
#[utoipa::path(post, path = "/api/user/avatar",
    security(("token" = [])),
    request_body(content = Image, content_type = "multipart/form-data"),
    responses((status = 200), (status = "4XX", body = UserError), (status = 500, body = ApiError))
)]
pub async fn avatar(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Option<uuid::Uuid>>,
    //Json(body): Json<Avatar>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    use diesel::prelude::*;

    let uuid = match user_id {
        Some(user_id) => user_id,
        None => return Err(ApiError::Query(UserError::Unauthorized)),
    };

    let user = db::execute(&state.database, move |conn| {
        users::table
            .into_boxed()
            .filter(users::id.eq(uuid))
            .first::<User>(conn)
            .optional()
    })
    .await?;

    let user = match user {
        Some(user) => user,
        None => return Err(ApiError::Query(UserError::NotFound)),
    };

    let data: Bytes = if let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| ApiError::ReadContent)?
    {
        /*if field.name().unwrap() != "file" {
            continue;
        }*/
        field.bytes().await.map_err(|_| ApiError::ReadContent)?
    } else {
        return Err(ApiError::ReadContent);
    };

    let avatars = db::execute(&state.database, move |conn| {
        users::table
            .into_boxed()
            .select(users::avatar)
            .get_results::<String>(conn)
    })
    .await?
    .into_iter()
    .filter(|avatar_hash| !avatar_hash.is_empty())
    .collect::<Vec<String>>();

    let avatar_id = sqids::Sqids::builder()
        .min_length(10)
        .blocklist(HashSet::from_iter(avatars.clone().into_iter()))
        .build()
        .unwrap()
        .encode(&[avatars.len() as u64])
        .unwrap();

    let reader = image::io::Reader::new(std::io::Cursor::new(data))
        .with_guessed_format()
        .unwrap();
    let format = reader.format().unwrap();
    let img = reader.decode().unwrap();

    img.save_with_format(
        Config::data_dir()
            .unwrap()
            .join("avatars")
            .join(avatar_id.clone()),
        format,
    )
    .unwrap();

    if !user.avatar.is_empty() {
        std::fs::remove_file(
            Config::data_dir()
                .unwrap()
                .join("avatars")
                .join(user.avatar.clone()),
        )
        .unwrap();
    }

    db::execute(&state.database, move |conn| {
        diesel::update(&user)
            .set(users::avatar.eq(avatar_id))
            .execute(conn)
    })
    .await?;

    Ok(())
}
