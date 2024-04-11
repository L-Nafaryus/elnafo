use deadpool_diesel::postgres::PoolError;
use deadpool_sync::InteractError;
use diesel::result::Error as DieselError;
use std::error::Error as StdError;
use std::fmt::Display;

#[derive(Debug, utoipa::ToSchema)]
pub enum DatabaseError {
    Connection,
    Interaction(InteractError),
    Operation(DieselError),
    Query(DieselError),
    Migration,
    Internal,
}

impl StdError for DatabaseError {}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Connection => write!(f, "Failed pool connection"),
            Self::Interaction(ref e) => e.fmt(f),
            Self::Operation(ref e) => e.fmt(f),
            Self::Query(ref e) => e.fmt(f),
            Self::Migration => write!(f, "Failed to run migrations"),
            Self::Internal => write!(f, "Internal error ..."),
        }
    }
}

impl From<PoolError> for DatabaseError {
    fn from(_: PoolError) -> Self {
        Self::Connection
    }
}

impl From<InteractError> for DatabaseError {
    fn from(e: InteractError) -> Self {
        Self::Interaction(e)
    }
}

impl From<DieselError> for DatabaseError {
    fn from(e: DieselError) -> Self {
        Self::Query(e)
    }
}
