use crate::app_error::AppError;
use crate::settings::RedisSettings;
use deadpool::managed::QueueMode;
use deadpool_redis::{Manager, PoolConfig, Runtime, Timeouts};

pub type RedisPool = deadpool_redis::Pool;

pub async fn init_redis(settings: &RedisSettings) -> Result<RedisPool, AppError> {
    let manager = Manager::new(settings.url.clone()).map_err(|e| AppError::RedisError(e.into()))?;
    let pool_config = PoolConfig {
        max_size: settings.max_conn,
        timeouts: Timeouts::wait_millis(10000),
        queue_mode: QueueMode::default(),
    };

    let pool = RedisPool::builder(manager)
        .config(pool_config)
        .runtime(Runtime::Tokio1)
        .build()
        .map_err(|e| AppError::RedisError(e.into()))?;

    Ok(pool)
    // let mut cfg = Config::from_url(settings.url);
    // let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
    // pool
}
