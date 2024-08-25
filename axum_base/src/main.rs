use basic::{env::Env, state::State};

mod basic;
mod route;
mod sample;

#[tokio::main]
async fn main() {
    let env = Env::new("dev.toml");
    basic::tracing::init(&env);
    let state = State::new(&env);
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
