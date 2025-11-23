use crate::config::MultiDataSourceConfig;
use crate::database::DbManager;

pub mod config;
pub mod conn;
pub mod database;
pub mod error;


pub async fn jieto_db_init(path: &str) -> anyhow::Result<DbManager> {
    MultiDataSourceConfig::from_toml(path).await
}
