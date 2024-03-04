use crate::schema;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub login: String,
    pub hashed_password: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
pub struct NewUser<'a> {
    pub login: &'a str,
    pub hashed_password: &'a str,
    pub name: &'a str,
    pub email: &'a str,
    pub is_admin: bool,
}
