use deadpool_diesel::postgres::PoolError;

#[derive(Debug)]
pub enum DatabaseError<E> {
    Connection(E),
    Interaction,
    Operation(E),
    Migration,
}

impl<E: std::fmt::Display> std::fmt::Display for DatabaseError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Connection(e) => write!(f, "Failed to connect to database: {}", e),
            Self::Interaction => write!(f, "Failed to interact with database"),
            Self::Operation(e) => write!(f, "Failed operation: {}", e),
            Self::Migration => write!(f, "Failed to run migrations"),
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for DatabaseError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Interaction | Self::Migration => None,
            Self::Connection(e) => Some(e),
            Self::Operation(e) => Some(e),
        }
    }
}

impl From<PoolError> for DatabaseError<PoolError> {
    fn from(e: PoolError) -> Self {
        Self::Connection(e)
    }
}

#[derive(Debug)]
pub enum UserError {
    Query,
    Exists,
    NotFound,
    HashPassword,
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Query => write!(f, "User query failed"),
            UserError::Exists => write!(f, "User already exists"),
            UserError::NotFound => write!(f, "User not found"),
            UserError::HashPassword => write!(f, "Failed to hash user password"),
        }
    }
}

impl std::error::Error for UserError {}

impl From<UserError> for DatabaseError<UserError> {
    fn from(e: UserError) -> Self {
        Self::Operation(e)
    }
}
