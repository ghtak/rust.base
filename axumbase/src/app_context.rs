use std::sync::Arc;

use axum::extract::FromRef;

use crate::{database::DatabasePool, settings::Settings};

#[derive(Debug, Clone)]
pub struct AppContext {
    pub settings: Arc<Settings>,
    pub database_pool: DatabasePool,
}

impl AppContext {
    pub fn new(settings: Settings, database_pool: DatabasePool) -> Self {
        AppContext {
            settings: Arc::new(settings),
            database_pool,
        }
    }
}

impl FromRef<AppContext> for DatabasePool {
    fn from_ref(input: &AppContext) -> Self {
        input.database_pool.clone()
    }
}
