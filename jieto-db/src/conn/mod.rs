use super::database::DataSource;
use super::error::DbError;
use async_trait::async_trait;

#[cfg(feature = "mysql")]
pub mod mysql;
#[cfg(feature = "postgres")]
pub mod postgres;
#[cfg(feature = "redis")]
pub mod redis;
#[cfg(feature = "sqlite")]
pub mod sqlite;

#[async_trait]
pub trait DatabaseInit {
    async fn init_datasource(&self) -> anyhow::Result<DataSource, DbError>;
}
