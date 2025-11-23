use super::super::conn::DatabaseInit;
use super::super::database::DataSource;
use super::super::error::DbError;
use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretBox};
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize, Debug, Default)]
pub struct SqliteSourceConfig {
    pub default: Option<bool>,
    pub name: String,
    pub url: String,
}

impl SqliteSourceConfig {
    fn create_uri(&self) -> SecretBox<str> {
        SecretBox::from(format!("sqlite://{}", self.url))
    }
}

#[async_trait]
impl DatabaseInit for SqliteSourceConfig {
    async fn init_datasource(&self) -> anyhow::Result<DataSource, DbError> {
        log::info!("[db][{}] Connecting to Sqlite database", self.name);
        let uri = self.create_uri();
        let pool = SqlitePool::connect_lazy(uri.expose_secret())?;
        Ok(DataSource::Sqlite {  pool })
    }
}
