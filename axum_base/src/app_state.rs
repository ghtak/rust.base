use bb8_redis::RedisConnectionManager;

use crate::basic::db::BasicDatabase;

pub type DatabaseDriver = sqlx::Sqlite;
pub type DBPool = BasicDatabase<DatabaseDriver>;
pub type RedisPool = bb8::Pool<RedisConnectionManager>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub database_pool: DBPool,
    pub redis_pool: Option<RedisPool>,
}

impl AppState {
    pub fn new(database_pool: DBPool, redis_pool: Option<RedisPool>) -> Self {
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
