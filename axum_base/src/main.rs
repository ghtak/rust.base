use app_state::{AppState, Database};
use basic::env::Env;

mod app_state;
mod basic;
mod oauth2sample;
mod route;
mod sample;

#[tokio::main]
async fn main() {
    let env = Env::new("dev.toml");
    basic::tracing::init(&env);
    tracing::info!(val=32, env=?env);

    let database = Database::builder()
        .env(env.clone())
        .connect()
        .await
        .expect("database build fail")
        .build();

    let state = AppState::new(database);
    let router = route::router(state);
    let listener = tokio::net::TcpListener::bind(env.server.address.as_str())
        .await
        .expect("listen fail");
    tracing::info!("run server {}", env.server.address.as_str());
    axum::serve(listener, router)
        .with_graceful_shutdown(basic::shutdown_signal())
        .await
        .expect("serve fail");
}
