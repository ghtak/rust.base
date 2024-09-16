use bb8_redis::RedisConnectionManager;

use crate::app_state::RedisPool;
use crate::basic::Result;

use super::env::Env;

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
