macro_rules! define_data_sources {
    (
        $(
            $variant:ident, $feature:literal, $pool_type:ty
        ),* $(,)?
    ) => {
        use std::collections::HashMap;
        use std::sync::Arc;
        use super::error::DbError;


        #[derive(Debug,Eq, Hash, PartialEq)]
        pub enum DefaultKey{
            $(
                #[cfg(feature = $feature)]
                $variant,
            )*
        }

        #[derive(Debug,Default,Clone)]
        pub struct  DbManager{
            inner:Arc<HashMap<String,DataSource>>,
            default_name:Arc<HashMap<DefaultKey,String>>
        }

        #[derive(Debug)]
         pub enum DataSource {
            None,
            $(
                #[cfg(feature = $feature)]
                $variant { pool: $pool_type },
            )*
        }

        impl DataSource {
            $(
                #[cfg(feature = $feature)]
                paste::paste! {
                    pub fn [<$variant:lower _pool>](&self) -> Result<$pool_type, DbError> {
                        match self {
                            DataSource::$variant { pool, .. } => Ok(pool.clone()),
                            _ => Err(DbError::UnDcp),
                        }
                    }
                }
            )*
        }

        $(
            #[cfg(feature = $feature)]
            impl TryFrom<DataSource> for $pool_type {
                type Error = DbError;

                fn try_from(value: DataSource) -> Result<Self, Self::Error> {
                    match value {
                        DataSource::$variant { pool, .. } => Ok(pool.clone()),
                        _ => Err(DbError::UnDcp),
                    }
                }
            }
        )*


        $(
            #[cfg(feature = $feature)]
            impl TryFrom<&DataSource> for $pool_type {
                type Error = DbError;

                fn try_from(value: &DataSource) -> Result<Self, Self::Error> {
                    match value {
                        DataSource::$variant { pool, .. } => Ok(pool.clone()),
                        _ => Err(DbError::UnDcp),
                    }
                }
            }
        )*

        impl DbManager {
              pub fn get(&self, name:&str) -> Option<&DataSource> {
                self.inner.get(name)
              }

             $(
                #[cfg(feature = $feature)]
                paste::paste! {
                    pub fn [<with_ $variant:lower>](&self, name:&str) ->anyhow::Result<$pool_type,DbError>{
                        self.get(name)
                            .ok_or_else(|| DbError::DataSourceNotFound(name.into()))?
                            .[<$variant:lower _pool>]()
                    }
                }

                #[cfg(feature = $feature)]
                paste::paste! {
                     pub fn [<with_ $variant:lower _default>](&self) ->anyhow::Result<$pool_type,DbError>{
                           let name = self.default_name
                            .get(&DefaultKey::[<$variant>])
                            .ok_or_else(|| DbError::DataSourceNotFound(stringify!($variant).into()))?;
                        self.[<with_ $variant:lower>](name)
                     }
                }
            )*
        }
    };
}

#[cfg(feature = "mysql")]
type MysqlPool = sqlx::MySqlPool;
#[cfg(feature = "postgres")]
type PgPool = sqlx::PgPool;
#[cfg(feature = "sqlite")]
type SqlitePool = sqlx::SqlitePool;
#[cfg(feature = "redis")]
type RedisPool = deadpool_redis::Pool;

define_data_sources! {
    Mysql, "mysql", MysqlPool,
    Sqlite, "sqlite", SqlitePool,
    Postgres, "postgres", PgPool,
    Redis, "redis", RedisPool,
}

impl DbManager {
    pub fn new(
        inner: HashMap<String, DataSource>,
        default_name: HashMap<DefaultKey, String>,
    ) -> Self {
        DbManager {
            inner: Arc::new(inner),
            default_name: Arc::new(default_name),
        }
    }
}
