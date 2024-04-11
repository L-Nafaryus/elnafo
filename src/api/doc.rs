use utoipa::{
    openapi::{
        security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme},
        Components,
    },
    Modify, OpenApi,
};

use super::errors;
use super::user;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::healthcheck,
        user::all,
        user::register,
        user::remove,
        user::login,
        user::logout,
        user::profile,
        user::current,
        user::avatar
    ),
    components(schemas(
        crate::db::errors::DatabaseError,
        user::UserError,
        user::schema::NewUser,
        user::schema::User,
        user::schema::RemoveUser,
        user::schema::LoginUser,
        user::schema::Avatar,
        user::schema::Image,
        errors::ApiError
    )),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if openapi.components.is_none() {
            openapi.components = Some(Components::new());
        }

        openapi.components.as_mut().unwrap().add_security_scheme(
            "token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}
