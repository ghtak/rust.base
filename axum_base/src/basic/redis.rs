use bb8_redis::RedisConnectionManager;

use crate::app_state::RedisPool;
use crate::basic::Result;

use super::env::Env;
use super::error::Error;

pub async fn init(env: &Env) -> Result<Option<RedisPool>> {
    if env.redis.enable == false {
        Ok(None)
    } else {
        let manager =
            RedisConnectionManager::new(format!("redis://{}:{}", env.redis.host, env.redis.port))?;
        let res = bb8::Pool::builder().build(manager).await?;
        Ok(Some(res))
    }
}

pub struct RedisRepository {
    redis_pool: RedisPool,
}

impl RedisRepository {
    pub fn new(redis_pool: RedisPool) -> Self {
        RedisRepository { redis_pool }
    }

    pub async fn set_string<'a>(&self, key: &'a str, value: &'a str) -> Result<()> {
        let mut conn = self
            .redis_pool
            .get()
            .await
            .map_err(|e| Error::Message(e.to_string()))?;
        redis::cmd("SET")
            .arg(key)
            .arg(value)
            .query_async(&mut *conn)
            .await
            .map_err(|e| Error::Message(e.to_string()))?;
        Ok(())
    }

    pub async fn get_string<'a>(&self, key: &'a str) -> Result<Option<String>> {
        let mut conn = self
            .redis_pool
            .get()
            .await
            .map_err(|e| Error::Message(e.to_string()))?;
        let value: Option<String> = redis::cmd("GET")
            .arg(key)
            .query_async(&mut *conn)
            .await
            .map_err(|e| Error::Message(e.to_string()))?;
        Ok(value)
    }
}
