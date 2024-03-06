pub mod models;
pub mod schema;

use deadpool_diesel::postgres::Manager;
pub use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::db::models::{NewUser, User};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db/migrations/");

pub fn create_pool(database_url: String) -> Pool {
    let manager = Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);

    Pool::builder(manager).build().unwrap()
}

pub async fn run_migrations(pool: &Pool) {
    let conn = pool.get().await.unwrap();
    conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
        .await
        .unwrap()
        .unwrap();
}

pub fn create_user(
    connection: &mut PgConnection,
    login: &str,
    hashed_password: &str,
    name: &str,
    email: &str,
    is_admin: bool,
) -> User {
    use crate::db::schema::users;

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
