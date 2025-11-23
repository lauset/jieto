use super::conn::DatabaseInit;
use serde::Deserialize;
use std::collections::HashMap;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[cfg(feature = "mysql")]
use super::conn::mysql::MySqlSourceConfig;
#[cfg(feature = "postgres")]
use super::conn::postgres::PostgresSourceConfig;
#[cfg(feature = "redis")]
use super::conn::redis::RedisSourceConfig;
#[cfg(feature = "sqlite")]
use super::conn::sqlite::SqliteSourceConfig;

use super::database::{DataSource, DbManager, DefaultKey};
use super::error::DbError;

#[derive(Deserialize, Debug, Default)]
pub(crate) struct MultiDataSourceConfig {
    #[cfg(feature = "sqlite")]
    #[serde(default)]
    pub sqlite: Vec<SqliteSourceConfig>,

    #[cfg(feature = "mysql")]
    #[serde(default)]
    pub mysql: Vec<MySqlSourceConfig>,

    #[cfg(feature = "postgres")]
    #[serde(default)]
    pub postgres: Vec<PostgresSourceConfig>,

    #[cfg(feature = "redis")]
    #[serde(default)]
    pub redis: Vec<RedisSourceConfig>,
}

macro_rules! define_into_data_sources {
    (
        $self:expr,
        $sources:ident,
        $dfu:ident,
        $( $Variant:ident ),* $(,)?
    ) => {
        $(
            paste::paste! {
    #[cfg(feature = $Variant:lower)]
    {
        let configs = &$self.[<$Variant:lower>];

        if !configs.is_empty() {
            let defaults: Vec<&String> = configs
                .iter()
                .filter(|c| c.default.unwrap_or_default())
                .map(|c| &c.name)
                .collect();

            let default_name = match defaults.len() {
                0 => configs[0].name.clone(),
                1 => defaults[0].clone(),
                _ => {
                    return Err(DbError::MultipleDefault {
                        db_type: stringify!($Variant).to_string(),
                        names: defaults.into_iter().cloned().collect(),
                    }.into());
                }
            };

            for config in configs {
                let ds = config.init_datasource().await?;
                $sources.insert(config.name.clone(), ds);
            }

            $dfu.insert(DefaultKey::$Variant, default_name);
        }
    }
}
        )*
    };
}

impl MultiDataSourceConfig {
    pub(crate) async fn from_toml(path: &str) -> anyhow::Result<DbManager> {
        let mut file = File::open(path).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        let config: MultiDataSourceConfig = toml::from_str(&contents)?;
        config.into_data_sources().await
    }

    pub(crate) async fn into_data_sources(self) -> anyhow::Result<DbManager> {
        let mut sources: HashMap<String, DataSource> = HashMap::new();
        let mut dfu: HashMap<DefaultKey, String> = HashMap::new();
        define_into_data_sources!(self, sources, dfu, Sqlite, Mysql, Postgres, Redis,);
        Ok(DbManager::new(sources, dfu))
    }
}
