use basic::{env::Env, state::BasicState};

mod basic;
mod oauth2sample;
mod route;
mod sample;


#[tokio::main]
async fn main() {
    let env = Env::new("dev.toml");
    basic::tracing::init(&env);
    tracing::info!(val=32, env=?env);

    let database = basic::db::Database::builder()
        .env(env.clone())
        .connect()
        .await.expect("database build fail")
        .build();
    // let wp = database.write_pool();
    // let wp2 = database.write_pool();
    
    // if std::ptr::eq(wp, wp2){
    //     tracing::info!("Eq Pools {:?} {:?}", wp, wp2);
    // }

    // let rp = database.read_pool();
    // let rp2 = database.read_pool();

    // if std::ptr::eq(rp, rp2){
    //     tracing::info!("Eq rPools {:?} {:?}", rp, rp2);
    // }
    
    let state = BasicState::new(&env, database);
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
