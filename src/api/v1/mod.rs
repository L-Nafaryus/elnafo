use crate::db::models::{FilteredUser, LoginUser, NewUser, RegisterUser, TokenClaims, User};
use crate::error_handle::internal_error;
use crate::state::AppState;
use argon2::{password_hash::SaltString, Argon2};
use argon2::{PasswordHash, PasswordHasher, PasswordVerifier};
use axum::extract::Request;
use axum::Extension;
use axum::{
    body::Body,
    extract::State,
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::cookie::{self, Cookie, SameSite};
use axum_extra::extract::CookieJar;
use diesel::{connection, prelude::*};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand_core::OsRng;
use std::sync::Arc;

pub async fn healthcheck() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "success",
        "message": "healthy"
    }))
}

pub async fn register_user(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    use crate::db::schema::{users, users::dsl};

    let connection = state.database.get().await.unwrap();
    let (login, email) = (body.login.clone(), body.email.clone());
    let user_exists = connection
        .interact(move |connection| {
            dsl::users
                .filter(dsl::login.eq(login).or(dsl::email.eq(email)))
                .select(User::as_select())
                .first(connection)
                .optional()
        })
        .await
        .map_err(internal_error)?
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                "status": "fail",
                "message": format!("Database error: {}", e)
                })),
            )
        })?;

    if user_exists.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(serde_json::json!({
            "status": "fail",
            "message": "Login or email already exists"
            })),
        ));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                "status": "fail",
                "message": format!("Error while hashing password: {}", e)
                })),
            )
        })
        .map(|hash| hash.to_string())?;

    let user = NewUser {
        login: body.login.to_string(),
        hashed_password: hashed_password,
        name: body.name,
        email: body.email,
        is_admin: body.is_admin,
    };

    let new_user = connection
        .interact(move |connection| {
            diesel::insert_into(users::table)
                .values(&user)
                .returning(User::as_returning())
                .get_result(connection)
        })
        .await
        .map_err(internal_error)?
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                "status": "fail",
                "message": format!("Database error: {}", e)
                })),
            )
        })?;

    let response = serde_json::json!({"status": "success", "data": serde_json::json!({"user": FilteredUser::from(&new_user)})});

    Ok(Json(response))
}

pub async fn login_user(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LoginUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    use crate::db::schema::{users, users::dsl};

    let connection = state.database.get().await.unwrap();
    let user = connection
        .interact(move |connection| {
            dsl::users
                .filter(dsl::email.eq(body.email))
                .select(User::as_select())
                .first(connection)
        })
        .await
        .map_err(internal_error)?
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                "status": "fail",
                "message": format!("Invalid login or email: {}", e)
                })),
            )
        })?;

    let is_valid = match PasswordHash::new(&user.hashed_password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
            "status": "fail",
            "message": "Invalid login, email or password"
            })),
        ));
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::try_minutes(60).unwrap()).timestamp() as usize;
    let claims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.config.jwt.secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response =
        Response::new(serde_json::json!({"status": "success", "token": token}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)
}

pub async fn logout_user() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::new(serde_json::json!({"status": "success"}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)
}

pub async fn jwt_auth(
    cookie_jar: CookieJar,
    State(state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });

    let token = token.ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
            "status": "fail",
            "message": "Cannot login without token"
            })),
        )
    })?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(state.config.jwt.secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"status":"fail","message":"Invalid token"})),
        )
    })?
    .claims;

    let user_id = uuid::Uuid::parse_str(&claims.sub).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"status":"fail","message":"Invalid token"})),
        )
    })?;

    use crate::db::schema::{users, users::dsl};

    let connection = state.database.get().await.unwrap();
    let user = connection
        .interact(move |connection| {
            dsl::users
                .filter(dsl::id.eq(user_id))
                .select(User::as_select())
                .first(connection)
                .optional()
        })
        .await
        .map_err(internal_error)?
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                "status": "fail",
                "message": format!("Database error: {}", e)
                })),
            )
        })?;

    let user = user.ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
            "status": "fail",
            "message": "The user belonging to this token no longer exists"
            })),
        )
    })?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

pub async fn me(
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(
        serde_json::json!({"status":"success","data":serde_json::json!({"user":FilteredUser::from(&user)})}),
    ))
}
