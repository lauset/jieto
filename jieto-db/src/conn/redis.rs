use super::super::conn::DatabaseInit;
use super::super::database::DataSource;
use super::super::error::DbError;
use async_trait::async_trait;
use deadpool_redis::{Config, Runtime};
use secrecy::{ExposeSecret, SecretBox};
use serde::Deserialize;
use urlencoding::encode;

#[derive(Deserialize, Debug, Default)]
pub struct RedisSourceConfig {
    pub default: Option<bool>,
    pub name: String,
    pub host: String,
    pub port: u16,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default)]
    pub db: u8,
    #[serde(default)]
    pub protocol: Option<String>,
}

impl RedisSourceConfig {
    fn create_uri(&self) -> SecretBox<str> {
        let mut url = if let Some(username) = self.username.as_ref() {
            format!("redis://{}", encode(username))
        } else {
            "redis://".to_string()
        };
        if let Some(password) = self.password.as_ref() {
            use std::fmt::Write;
            write!(url, ":{}", encode(password)).unwrap();
        }
        use std::fmt::Write;
        write!(url, "@{}:{}", self.host, self.port).unwrap();
        write!(url, "/{}", self.db).unwrap();

        if let Some(proto) = self.protocol.as_ref() {
            write!(url, "?protocol={}", proto).unwrap();
        }
        SecretBox::from(url)
    }
}

#[async_trait]
impl DatabaseInit for RedisSourceConfig {
    async fn init_datasource(&self) -> anyhow::Result<DataSource, DbError> {
        log::info!("[db][{}] Connecting to Redis database", self.name);
        let uri = self.create_uri();
        let cfg = Config::from_url(uri.expose_secret());
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
        Ok(DataSource::Redis { pool })
    }
}
