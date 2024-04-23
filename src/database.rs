use actix_web::web;
use sqlx::PgPool;
use crate::config::{Config, CONFIG};
#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(field_identifier, rename_all = "lowercase")]
#[serde(untagged)]
pub enum DatabaseConnection {
    Cockroach,
    Mysql,
    Postgres,
    Sqlite,
}

pub type CockroachPool = PgPool;
pub type MysqlPool = MysqlPool;
pub type PostgresPool = PgPool;
pub type SqlitePool = SqlitePool;

#[cfg(feature = "cockroach")]
pub type PoolType = CockroachPool;

#[cfg(feature = "mysql")]
pub type PoolType = MysqlPool;

#[cfg(feature = "postgres")]
pub type PoolType = PostgresPool;

#[cfg(feature = "sqlite")]
pub type PoolType = SqlitePool;

pub enum InferPool {
    Cockroach(CockroachPool),
    Mysql(MysqlPool),
    Postgres(PostgresPool),
    Sqlite(SqlitePool),
}

impl InferPool {
    pub async fn init_pool(config: Config) -> Result<Self, sqlx::Error> {
        match config.database {
            DatabaseConnection::Cockroach => {
                let pool = PgPool::connect(&config.database_url).await?;
                Ok(InferPool::Cockroach(pool))
            }
            DatabaseConnection::Mysql => {
                let pool = MysqlPool::connect(&config.database_url).await?;
                Ok(InferPool::Mysql(pool))
            }
            DatabaseConnection::Postgres => {
                let pool = PgPool::connect(&config.database_url).await?;
                Ok(InferPool::Postgres(pool))
            }
            DatabaseConnection::Sqlite => {
                let pool = PgPool::connect(&config.database_url).await?;
                Ok(InferPool::Sqlite(pool))
            }
        }
    }
}

pub async fn add_pool(config: &mut web::ServiceConfig) {
    let pool = InferPool::init_pool(CONFIG.clone()).await.expect("Failed to create connection pool");
    match pool {
        InferPool::Cockroach(cockroach_pool) => config.app_data(cockroach_pool),
        InferPool::Mysql(mysql_pool) => config.app_data(mysql_pool),
        InferPool::Postgres(postgres_pool) => config.app_data(postgres_pool),
        InferPool::Sqlite(sqlite_pool) => config.app_data(sqlite_pool),
    };
}