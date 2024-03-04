pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::{NewUser, User};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("Missed DATABASE_URL");

    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}

pub fn create_user(
    connection: &mut PgConnection,
    login: &str,
    hashed_password: &str,
    name: &str,
    email: &str,
    is_admin: bool,
) -> User {
    use crate::schema::users;

    let new_user = NewUser {
        login,
        hashed_password,
        name,
        email,
        is_admin,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(connection)
        .expect("Error creating new user")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        unimplemented!();
    }
}
