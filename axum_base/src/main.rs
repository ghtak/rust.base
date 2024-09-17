use app_state::{AppState, DatabaseDriver};
use basic::env::Env;

mod app_state;
mod basic;
mod oauth2sample;
mod route;
mod sample;

#[tokio::main]
async fn main() {
    let env = Env::new("dev.toml");
    basic::tracing::init_tracing(&env);
    tracing::info!(test_value=81, env=?env);
    let db_pool = basic::db::create_pool::<DatabaseDriver>(&env)
        .await
        .expect("can't create db pool");
    let redis_pool = basic::redis::create_pool(&env)
        .await
        .expect("can't create redis pool");

    let state = AppState::new(db_pool, redis_pool);
    let router = route::router(state);
    let listener = tokio::net::TcpListener::bind(env.server.address.as_str())
        .await
        .expect("listen fail");
    tracing::info!("run server {}", env.server.address.as_str());

    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(basic::shutdown_signal())
        .await
        .expect("serve fail");
}
