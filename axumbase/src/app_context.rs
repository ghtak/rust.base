use std::sync::Arc;

use axum::extract::FromRef;

use crate::{database::DatabasePool, redis::RedisPool, settings::Settings};

#[derive(Debug, Clone)]
pub struct AppContext {
    pub settings: Arc<Settings>,
    pub database_pool: DatabasePool,
    pub redis_pool: RedisPool,
}

impl AppContext {
    pub fn new(settings: Settings, database_pool: DatabasePool, redis_pool: RedisPool) -> Self {
        AppContext {
            settings: Arc::new(settings),
            database_pool,
            redis_pool,
        }
    }
}

impl FromRef<AppContext> for DatabasePool {
    fn from_ref(input: &AppContext) -> Self {
        input.database_pool.clone()
    }
}

impl FromRef<AppContext> for RedisPool {
    fn from_ref(input: &AppContext) -> Self {
        input.redis_pool.clone()
    }
}
