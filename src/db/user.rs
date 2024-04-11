use crate::db::schema::users;
use diesel::{
    dsl::{AsSelect, SqlTypeOf},
    pg::Pg,
    prelude::*,
};

#[derive(serde::Serialize, Queryable, Selectable, Clone, Identifiable, AsChangeset)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: uuid::Uuid,
    pub login: String,
    pub hashed_password: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub avatar: String,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub login: String,
    pub hashed_password: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub avatar: String,
}

#[allow(dead_code)]
type SqlType = SqlTypeOf<AsSelect<User, Pg>>;

#[allow(dead_code)]
type BoxedQuery<'a> = users::BoxedQuery<'a, Pg, SqlType>;
