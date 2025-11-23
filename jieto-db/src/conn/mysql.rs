use std::cmp::max;
use std::time::Duration;
use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretBox};
use serde::Deserialize;
use sqlx::mysql::MySqlPoolOptions;
use urlencoding::encode;
use super::super::conn::DatabaseInit;
use super::super::error::DbError;
use super::super::database::DataSource;

#[derive(Deserialize, Debug,Default)]
pub struct MySqlSourceConfig {
    pub default:Option<bool>,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl MySqlSourceConfig{
    fn create_uri(&self)->SecretBox<str>{
        SecretBox::from(format!(
            "mysql://{}:{}@{}:{}/{}",
            encode(&self.username),
            encode(&self.password),
            self.host,
            self.port,
            encode(&self.database)
        ))
    }
}

#[async_trait]
impl DatabaseInit for MySqlSourceConfig {
    async fn init_datasource(&self) -> anyhow::Result<DataSource, DbError> {
        log::info!(
        "[db] Connecting to Mysql database: {}", self.name
    );
        let uri = self.create_uri();
        let cpus = num_cpus::get() as u32;
        let pool = MySqlPoolOptions::new()
            .min_connections(max(2, cpus / 2))
            .max_connections(max(20, cpus * 2))
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(300))
            .max_lifetime(Duration::from_secs(3600 * 24))
            .connect_lazy(uri.expose_secret())?;
        Ok(DataSource::Mysql{
            pool,
        })
    }
}
