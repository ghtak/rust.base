mod app_context;
mod app_error;
mod database;
mod extract_ext;
mod logging;
mod redis;
mod routes;
mod settings;

use crate::logging::init_logging;
use app_context::AppContext;
use database::init_database;
use redis::init_redis;
use routes::routes;
use settings::load_settings;
use sqlx::Executor;

#[tokio::main]
async fn main() {
    let settings = load_settings().unwrap();
    let _guards = init_logging(&settings.log);
    let db_pool = init_database(&settings.database).await.unwrap();
    db_pool
        .execute(
            "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT UNIQUE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)",
        )
        .await
        .unwrap();

    let redis_pool = init_redis(&settings.redis).await.unwrap();
    let listener = tokio::net::TcpListener::bind(settings.server.address().as_str())
        .await
        .unwrap();
    axum::serve(listener, routes(AppContext::new(db_pool, redis_pool)))
        .await
        .unwrap();
}
