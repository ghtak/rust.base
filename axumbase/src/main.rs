mod app_context;
mod app_error;
mod database;
mod extract_ext;
mod logging;
mod routes;
mod settings;

use crate::logging::init_logging;
use app_context::AppContext;
use database::DatabasePoolOptions;
use routes::routes;
use settings::load_settings;
use sqlx::Executor;

#[tokio::main]
async fn main() {
    let settings = load_settings().unwrap();
    let _guards = init_logging(&settings.log);
    let pool = DatabasePoolOptions::new()
        .max_connections(settings.database.max_conn)
        .connect(&settings.database.url)
        .await
        .unwrap();

    let app_context = AppContext::new(settings, pool);
    app_context
        .database_pool
        .execute(
            "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT UNIQUE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        )
        .await
        .unwrap();

    let listener = tokio::net::TcpListener::bind(app_context.settings.server.address().as_str())
        .await
        .unwrap();
    axum::serve(listener, routes(app_context)).await.unwrap();
}
