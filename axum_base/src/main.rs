use tokio::signal;
mod basic;
mod route;
mod sample;


#[tokio::main]
async fn main() {
    let env = basic::env::Env::new("dev.toml");
    basic::tracing::init(&env);
    let state = basic::state::State {};
    tokio::spawn(async move {
        tracing::info!("run server {}", env.server.address.as_str());
        let router = route::router(state);
        let listener = tokio::net::TcpListener::bind(env.server.address.as_str())
            .await
            .expect("listen fail");
        axum::serve(listener, router).await.expect("serve fail");
    });
    signal::ctrl_c().await.expect("signal fail");
}
