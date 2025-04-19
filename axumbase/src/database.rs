use crate::{app_error::AppError, settings::DatabaseSettings};


pub trait DatabaseTrait {
    type Impl;
    type Pool;
    type PoolOptions;
}

pub struct SqliteDatabase;

impl DatabaseTrait for SqliteDatabase {
    type Impl = sqlx::sqlite::Sqlite;
    type Pool = sqlx::sqlite::SqlitePool;
    type PoolOptions = sqlx::sqlite::SqlitePoolOptions;
}

pub struct PostgresDatabase;

impl DatabaseTrait for PostgresDatabase {
    type Impl = sqlx::postgres::Postgres;
    type Pool = sqlx::postgres::PgPool;
    type PoolOptions = sqlx::postgres::PgPoolOptions;
}

#[cfg(feature = "sqlite")]
pub type Database = SqliteDatabase;

#[cfg(feature = "postgres")]
pub type Database = PostgresDatabase;


pub type DatabasePool = <Database as DatabaseTrait>::Pool;
pub type DatabasePoolOptions = <Database as DatabaseTrait>::PoolOptions;

pub async fn init_database(settings: &DatabaseSettings) -> Result<DatabasePool, AppError> {
    DatabasePoolOptions::new()
        .max_connections(settings.max_conn)
        .connect(&settings.url)
        .await
        .map_err(|e| AppError::DatabaseError(e.into()))
}