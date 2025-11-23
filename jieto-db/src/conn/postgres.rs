use super::super::conn::DatabaseInit;
use super::super::database::DataSource;
use super::super::error::DbError;
use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretBox};
use serde::Deserialize;
use sqlx::PgPool;
use urlencoding::encode;

#[derive(Deserialize, Debug, Default)]
pub struct PostgresSourceConfig {
    pub default: Option<bool>,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    #[serde(default)]
    pub ssl_mode: Option<String>,
    #[serde(default)]
    pub schema: Option<String>,
    #[serde(default)]
    pub application_name: Option<String>,
}

impl PostgresSourceConfig {
    fn create_uri(&self) -> SecretBox<str> {
        let mut u = format!(
            "postgres://{}:{}@{}:{}/{}",
            encode(&self.username),
            encode(&self.password),
            self.host,
            self.port,
            encode(&self.database)
        );

        let mut params = vec![];

        if let Some(ssl) = self.ssl_mode.as_ref() {
            params.push(format!("sslmode={}", ssl));
        }

        if let Some(sch) = self.schema.as_ref() {
            params.push(format!("options=--search_path%3D{}", sch));
        }

        if let Some(app_name) = self.application_name.as_ref() {
            params.push(format!("application_name={}", app_name));
        }

        if !params.is_empty() {
            u.push('?');
            u.push_str(&params.join("&"));
        }

        SecretBox::from(u)
    }
}

#[async_trait]
impl DatabaseInit for PostgresSourceConfig {
    async fn init_datasource(&self) -> anyhow::Result<DataSource, DbError> {
        log::info!("[db] Connecting to Postgres database: {}", self.name);
        let uri = self.create_uri();
        let pool = PgPool::connect_lazy(uri.expose_secret())?;
        Ok(DataSource::Postgres { pool })
    }
}
