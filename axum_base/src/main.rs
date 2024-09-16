use app_state::{AppState, DBPool};
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
 
    let database = DBPool::builder(env.clone())
        .connect()
        .await
        .expect("database build fail")
        .build();

    let redis_pool = basic::redis::init(&env).await.expect("msg");

    let state = AppState::new(database, redis_pool);
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
