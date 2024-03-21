pub mod errors;
pub mod schema;
pub mod user;

use deadpool_diesel::postgres::Manager;
pub use deadpool_diesel::postgres::Pool;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use errors::DatabaseError;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db/migrations/");

pub fn create_pool(database_url: String) -> Pool {
    let manager = Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);

    Pool::builder(manager).build().unwrap()
}

pub async fn run_migrations(pool: &Pool) -> Result<(), DatabaseError<impl std::error::Error>> {
    let connection = pool.get().await.map_err(DatabaseError::Connection)?;
    connection
        .interact(move |connection| connection.run_pending_migrations(MIGRATIONS).map(|_| ()))
        .await
        .map_err(|_| DatabaseError::Interaction)?
        .map_err(|_| DatabaseError::Migration)?;
    Ok(())
}
