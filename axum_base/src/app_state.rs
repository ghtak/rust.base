use axum::{async_trait, extract::{FromRef, FromRequestParts}, http::request::Parts};
use bb8::PooledConnection;
use bb8_redis::RedisConnectionManager;

use crate::basic::{self, db::BasicDatabase, depends::Depends, redis::RedisRepository, Result};

pub type DatabaseDriver = sqlx::Sqlite;
pub type DBPool = BasicDatabase<DatabaseDriver>;
pub type RedisPool = bb8::Pool<RedisConnectionManager>;
pub type RedisConnection = PooledConnection<'static, RedisConnectionManager>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub database_pool: DBPool,
    pub redis_pool: RedisPool,
}

impl AppState {
    pub fn new(database_pool: DBPool, redis_pool: RedisPool) -> Self {
        AppState {
            database_pool,
            redis_pool,
        }
    }
}

impl Clone for DBPool {
    fn clone(&self) -> Self {
        self.clone_internal()
    }
}

impl FromRef<AppState> for DBPool {
    fn from_ref(input: &AppState) -> Self {
        input.database_pool.clone()
    }
}

impl FromRef<AppState> for RedisPool {
    fn from_ref(input: &AppState) -> Self {
        // let p = input.redis_pool.as_ref().map(|x| x.clone());
        // p.unwrap()
        input.redis_pool.clone()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Depends<RedisConnection>
where
    S: Send + Sync,
    RedisPool: FromRef<S>,
{
    type Rejection = basic::error::Error;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self> {
        let pool = RedisPool::from_ref(state);
        let conn = pool
            .get_owned()
            .await
            .map_err(|e| basic::error::Error::Message(e.to_string()))?;
        Ok(Self(conn))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Depends<RedisRepository>
where
    S: Send + Sync,
    RedisPool: FromRef<S>,
{
    type Rejection = basic::error::Error;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self> {
        Ok(Self(RedisRepository::new(RedisPool::from_ref(state))))
    }
}