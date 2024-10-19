use bb8::PooledConnection;
use bb8_redis::RedisConnectionManager;
use error_stack::ResultExt;
use redis::AsyncCommands;

use crate::app_state::{RedisConnection, RedisPool};

use super::env::Env;
use super::error::{AppError, AppResult};

pub async fn create_pool(env: &Env) -> AppResult<RedisPool> {
    let manager =
        RedisConnectionManager::new(format!("redis://{}:{}", env.redis.host, env.redis.port))
            .change_context(AppError::InitError("redis connection manager".to_string()))?;
    Ok(bb8::Pool::builder()
        .build(manager)
        .await
        .change_context(AppError::InitError("redis pool".to_string()))?)
}

pub struct RedisRepository {
    redis_pool: RedisPool,
}

impl RedisRepository {
    pub fn new(redis_pool: RedisPool) -> Self {
        RedisRepository { redis_pool }
    }

    async fn get_connection(&self) -> AppResult<PooledConnection<'_, RedisConnectionManager>> {
        Ok(self
            .redis_pool
            .get()
            .await
            .change_context(AppError::PortError("redis connection".to_string()))?)
    }

    pub async fn set_string<'a>(&self, key: &'a str, value: &'a str) -> AppResult<()> {
        let mut conn = self.get_connection().await?;
        conn.set(key, value)
            .await
            .change_context(AppError::PortError("redis set".to_string()))?;
        // redis::cmd("SET")
        //     .arg(key)
        //     .arg(value)
        //     .query_async(&mut *conn)
        //     .await
        //     .change_context(AppError::PortError("redis set".to_string()))?;
        Ok(())
    }

    pub async fn get_string<'a>(&self, key: &'a str) -> AppResult<Option<String>> {
        let mut conn = self.get_connection().await?;
        let value: Option<String> = conn
            .get(key)
            .await
            .change_context(AppError::PortError("redis get".to_string()))?;
        // let value: Option<String> = redis::cmd("GET")
        //     .arg(key)
        //     .query_async(&mut *conn)
        //     .await
        //     .change_context(AppError::PortError("redis get".to_string()))?;
        Ok(value)
    }
}
