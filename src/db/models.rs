use crate::db::schema;
use diesel::prelude::*;

#[derive(serde::Serialize, Queryable, Selectable, Clone)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: uuid::Uuid,
    pub login: String,
    pub hashed_password: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = schema::users)]
pub struct NewUser {
    pub login: String,
    pub hashed_password: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(serde::Deserialize)]
pub struct RegisterUser {
    pub login: String,
    pub password: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(serde::Serialize)]
pub struct FilteredUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
}

impl FilteredUser {
    pub fn from(user: &User) -> Self {
        FilteredUser {
            id: user.id.to_string(),
            name: user.name.to_owned(),
            email: user.email.to_owned(),
            is_admin: user.is_admin,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}
