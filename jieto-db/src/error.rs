use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Data source '{0}' not found")]
    DataSourceNotFound(String),

    #[error("Data source '{0}' not default")]
    DataSourceNotDefault(String),

    #[error("Multiple 'default = true' found for {db_type}: {names:?}")]
    MultipleDefault { db_type: String, names: Vec<String> },

    #[error("Connection pool not initialized for data source '{0}'")]
    PoolNotConfigured(String),

    #[error("Cannot convert data source to target database type")]
    UnDcp,

    #[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
    #[error("Database operation failed: {source}")]
    Sqlx {
        #[from]
        source: sqlx::Error,
    },

    #[cfg(feature = "redis")]
    #[error("Redis operation failed: {source}")]
    RedisCommand {
        #[from]
        source: deadpool_redis::CreatePoolError,
    },
    #[cfg(feature = "redis")]
    #[error("[Redis]{source}")]
    Redis{
        #[from]
        source: deadpool_redis::redis::RedisError,
    }
}
