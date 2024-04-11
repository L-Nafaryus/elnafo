pub mod errors;
pub mod schema;
pub mod user;

use deadpool_diesel::postgres::Manager;
pub use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use errors::DatabaseError;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db/migrations/");

pub fn create_pool(database_url: String) -> Pool {
    let manager = Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);

    Pool::builder(manager).build().unwrap()
}

pub async fn execute<F, T>(pool: &Pool, f: F) -> Result<T, DatabaseError>
where
    F: FnOnce(&mut PgConnection) -> Result<T, diesel::result::Error> + Send + 'static,
    T: Send + 'static,
{
    let connection = pool.get().await.map_err(|_| DatabaseError::Connection)?;

    connection
        .interact(move |connection| f(connection))
        .await
        .map_err(DatabaseError::Interaction)?
        .map_err(DatabaseError::Query)
}

pub async fn run_migrations(pool: &Pool) -> Result<(), DatabaseError> {
    execute(pool, move |connection| {
        Ok(connection
            .run_pending_migrations(MIGRATIONS)
            .map(|_| ())
            .map_err(|_| DatabaseError::Migration))
    })
    .await?
}
